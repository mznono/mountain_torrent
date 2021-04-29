use toml;
use std::fs::File;
use std::io::Read;
use serde_derive::{Deserialize,Serialize};
use serde_json;

use super::models;
use paho_mqtt as mqtt;
use std::{collections::HashMap, process, thread, time::Duration,sync::{RwLock,RwLockWriteGuard,RwLockReadGuard}};

#[derive(Debug,Deserialize)]
pub struct Config {
    qos:Option<i32>,
    mqtt_host:Option<String>,
    client_id:Option<String>,
    user_name:Option<String>,
    password:Option<String>,
}

impl Config {
    pub fn new() -> Self {
        deser_toml()
    }
    pub fn qos(&self) -> i32 {
        match self.qos {
            Some(q) => q,
            None => 1,
        }
    }
    
    pub fn host(&self) -> &str {
        match self.mqtt_host.as_ref() {
            Some(h) => &h,
            None => "",
        }
    }

    pub fn client_id(&self) -> &str {
        match self.client_id.as_ref() {
            Some(id) => &id,
            None => "",
        }
    }

    pub fn user_name(&self) -> &str {
        match self.user_name.as_ref() {
            Some(u) => &u,
            None => "",
        }
    }

    pub fn password(&self) -> &str {
        match self.password.as_ref() {
            Some(p) => &p,
            None => "",
        }
    }
    
}

fn deser_toml() -> Config {
    let mut toml_str = String::new();
    File::open("Config.toml")
        .and_then(|mut f| f.read_to_string(&mut toml_str))
        .unwrap();
    
    toml::from_str(&toml_str).unwrap()    
}
const DEPTH_POINT:[&str;33] = ["44271","44275","44279","44283","44287","44291","44295","44299","44303",
                    "44307","44311","44315","44319","44323","44327","44331","44335","44339",
                    "44343","44347","44351","44355","44359","44363","44367","44371","44375",
                    "44379","44383","44387","44391","44395","44399"
];

#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadData {
    #[serde(rename="dataPoints")]
    data_points:Vec<DataPoint>,
    #[serde(rename="devName")]
    dev_name:String
}

#[derive(Serialize, Deserialize, Debug)]
struct DataPoint {
    #[serde(rename="pointId")]
    point_id:String,
    value:String,
    #[serde(rename="slaveIndex")]
    slave_index:String,
    #[serde(rename="slaveAddr")]
    slave_addr:String,
}

impl From<String> for PayloadData {
    fn from(payload:String) -> PayloadData {
        let payload_data:PayloadData =serde_json::from_str(&payload)
        .unwrap_or(PayloadData{
            data_points:vec![],
            dev_name:"".to_string(),
        });
        payload_data
    }
}

impl DataPoint {
    fn is_rainfall(&self) -> bool {
        if self.point_id == "44266" {
            return true;
        }
        false
    }
    fn is_depth(&self) -> bool {
        DEPTH_POINT.contains(&self.point_id.as_str())
    }
    fn get_value(&self) -> f32 {
        self.value.parse().unwrap_or(0.0f32)
    }
}

type DataHasp = RwLock<HashMap<String, f32>>;

fn on_connect_success(cli: &mqtt::AsyncClient, _msgid: u16) {
    println!("Connection succeeded");
    if let Ok(topics) = models::topics() {        
        let qos = Config::new().qos();
        let qos = vec![qos; topics.len()];
        cli.subscribe_many(&topics, &qos);
    }    
}

fn on_connect_failure(cli: &mqtt::AsyncClient, _msgid: u16, rc: i32) {
    println!("Connection attempt failed with error code {}.\n", rc);
    thread::sleep(Duration::from_millis(2500));
    cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
}

fn on_message(cli: &mqtt::AsyncClient, msg: Option<mqtt::Message>) {       
    if let Some(msg) = msg {
        let payload_str = msg.payload_str().to_string();
        let topic = msg.topic();
        let payload = PayloadData::from(payload_str);
        let conn = models::db_connection();
        if conn.is_err() {
            return;
        }
        let conn = conn.unwrap();
        let dev_id = device_id_in_topic(topic);
        let dev_id = models::get_id_by_deviceid(&conn, dev_id.to_string());
        if dev_id.is_err() {
            return;
        }
        let dev_id = dev_id.unwrap();
        for point in payload.data_points {
            if point.is_rainfall() {
                store_rainfall(dev_id, &point, cli, topic);                             
                
            }
            if point.is_depth() {
                store_water_depth(dev_id, &point);
            }                     
        }                      
    }
    
    
}
fn store_rainfall(dev_id:i32,point:&DataPoint,cli: &mqtt::AsyncClient,topic:&str) {
    {
        let new_value = point.get_value();
        let hasp_read_data = get_user_read_data(cli);
        let value = hasp_read_data.get(topic).unwrap_or(&new_value);        
        let rainfall_value = (new_value-value).max(0.0);        
        models::new_rainfall(dev_id, rainfall_value).unwrap_or_default();
        models::new_calculation(dev_id, rainfall_value).unwrap_or_default();
          
    }                   
    
    {
        let mut hasp_write_data = get_user_write_data(cli);
        hasp_write_data.insert(topic.to_string(), point.get_value());
    } 

}

fn store_water_depth(dev_id:i32,poit:&DataPoint) {
    let value = poit.get_value();    
    models::new_water_depth(dev_id, value).unwrap_or_default();

}

fn device_id_in_topic(topic:&str) -> &str {
    topic.split_at(15).1
}

fn get_user_write_data(cli: &mqtt::AsyncClient) -> RwLockWriteGuard<HashMap<String, f32>> {
    let data = cli.user_data().unwrap();
    let lock = data.downcast_ref::<DataHasp>().unwrap();
    lock.write().unwrap()    
}

fn get_user_read_data(cli: &mqtt::AsyncClient) -> RwLockReadGuard<HashMap<String, f32>> {
    let data = cli.user_data().unwrap();
    let lock = data.downcast_ref::<DataHasp>().unwrap();
    lock.read().unwrap()
}

pub fn run_mqtt_client() {
    let config = Config::new();
    let hash_datas: HashMap<String, f32> = HashMap::new();    
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(config.host())
        .client_id(config.client_id())
        .user_data(Box::new(RwLock::new(hash_datas)))                
        .finalize();

    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    cli.set_connected_callback(|_cli| {
        println!("Connected.");
    });

    cli.set_connection_lost_callback(|cli| {
        println!("Connection lost. Attempting reconnect.");
        thread::sleep(Duration::from_millis(2500));
        cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
    });

    cli.set_message_callback(on_message);

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .user_name(config.user_name())
        .password(md5_password(config.password()))
        .finalize();

    cli.connect_with_callbacks(conn_opts, on_connect_success, on_connect_failure);
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}

fn md5_password(password: &str) -> String {
    let digest = md5::compute(password);
    format!("{:?}", digest)
}
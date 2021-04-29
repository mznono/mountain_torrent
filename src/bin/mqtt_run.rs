
use mountain_torrents::mqtt_client::*;
//use chrono::prelude::*;


fn main() {
    
    //let utc:DateTime<Utc> = Utc::now();
    //let local:DateTime<Local> = Local::now();
    //let navice_time = NaiveDateTime::from_timestamp((local.timestamp()+8*3600)/1800*1800, 0);  
    //println!("naive:{},utc:{}",navice_time.format("%Y-%m-%d %H:%M:%S"),utc.timestamp()) ;
    run_mqtt_client();
}
use bigdecimal::BigDecimal;
use bigdecimal::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::dsl::{sum,avg};
use chrono::prelude::*;
use dotenv::dotenv;
use std::env;

use super::error::Error;
use super::schema::{devices,rainfalls,water_depths,calculations};

#[derive(Debug, Queryable,Identifiable)]
#[table_name="devices"]
pub struct Device {
    pub id:i32,
    pub region:String,
    pub name:String,
    pub device_id:String,
    pub dike_height:BigDecimal,
    pub half_hour_design:BigDecimal,
    pub one_hour_design:BigDecimal,
    pub one_half_hour_design:BigDecimal,
    pub two_hour_design:BigDecimal,
    pub three_design:BigDecimal,
    pub stream_width:Option<BigDecimal>,
    pub rainfall_area:Option<BigDecimal>
}

impl Device {
    pub fn height_def(&self) -> f32 {
        decimal_to_f32(&self.dike_height)
    }
    pub fn half_hour_def(&self) -> f32 {
        decimal_to_f32(&self.half_hour_design)
    }
    pub fn one_hour_def(&self) -> f32 {
        decimal_to_f32(&self.one_hour_design)
    }
    pub fn one_half_hour_def(&self) -> f32 {
        decimal_to_f32(&self.one_half_hour_design)
    }
    pub fn two_hour_def(&self) -> f32 {
        decimal_to_f32(&self.two_hour_design)
    }
    pub fn three_hour_def(&self) -> f32 {
        decimal_to_f32(&self.three_design)
    }
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Device)]
pub struct Rainfall {
    pub id:i32,
    pub device_id:i32,
    pub value:BigDecimal,    
    pub create_time:NaiveDateTime,
    pub half_hour:i64,
    pub one_hour:i64,
    pub one_half_hour:i64,
    pub two_hour:i64,
    pub three_hour:i64,    
}

#[derive(Insertable)]
#[table_name="rainfalls"]
struct NewRainfall {
    device_id:i32,
    value:BigDecimal,    
    create_time:NaiveDateTime,
    half_hour:i64,
    one_hour:i64,
    one_half_hour:i64,
    two_hour:i64,
    three_hour:i64,     
}
impl NewRainfall {
    pub fn new(device_id:i32,value:f32) -> NewRainfall {
        let (now_stamp,half_stamp,one_stamp,one_half_stamp,two_stamp,three_stamp) = six_timestamps();
        let naive_time = NaiveDateTime::from_timestamp(now_stamp, 0);
        NewRainfall {
            device_id,
            value:BigDecimal::from(value),
            create_time:naive_time,
            half_hour:half_stamp,
            one_hour:one_stamp,
            one_half_hour:one_half_stamp,
            two_hour:two_stamp,
            three_hour:three_stamp,
        }
    }
}

#[derive(Insertable)]
#[table_name="water_depths"]
struct NewWaterDepth {
    device_id:i32,
    value:BigDecimal,
    flow_value:BigDecimal,    
    create_time:NaiveDateTime,
    half_hour:i64,
    one_hour:i64,
    one_half_hour:i64,
    two_hour:i64,
    three_hour:i64,    
}

impl NewWaterDepth {
    fn new(device_id:i32,value:f32,flow_value:f32) -> NewWaterDepth {
        let (now_stamp,half_stamp,one_stamp,one_half_stamp,two_stamp,three_stamp) = six_timestamps();
        let naive_time = NaiveDateTime::from_timestamp(now_stamp, 0);        
        NewWaterDepth {
            device_id,
            value:BigDecimal::from(value),
            flow_value:BigDecimal::from(flow_value),
            create_time:naive_time,
            half_hour:half_stamp,
            one_hour:one_stamp,
            one_half_hour:one_half_stamp,
            two_hour:two_stamp,
            three_hour:three_stamp,
        }
    }
}

#[derive(Insertable)]
#[table_name="calculations"]
struct NewCalculation {
    device_id:i32,
    storage:BigDecimal,
    wi:BigDecimal,
    quantity:BigDecimal,    
    create_time:NaiveDateTime,
    half_hour:i64,
    one_hour:i64,
    one_half_hour:i64,
    two_hour:i64,
    three_hour:i64,    
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Device)]
pub struct Calculation {
    pub id:i32,
    device_id:i32,
    storage:BigDecimal,
    wi:BigDecimal,
    quantity:BigDecimal, 
    create_time:NaiveDateTime,
    half_hour:i64,
    one_hour:i64,
    one_half_hour:i64,
    two_hour:i64,
    three_hour:i64,       
}

impl NewCalculation {
    fn new(device_id:i32,storage:f32,wi:f32,quantity:f32) -> NewCalculation {
        let (now_stamp,half_stamp,one_stamp,one_half_stamp,two_stamp,three_stamp) = six_timestamps();
        let naive_time = NaiveDateTime::from_timestamp(now_stamp, 0);        
        NewCalculation {
            device_id,
            storage:BigDecimal::from(storage),
            wi:BigDecimal::from(wi),
            quantity:BigDecimal::from(quantity),
            create_time:naive_time,
            half_hour:half_stamp,
            one_hour:one_stamp,
            one_half_hour:one_half_stamp,
            two_hour:two_stamp,
            three_hour:three_stamp,
        }
    }
}

pub fn db_connection() -> Result<PgConnection, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .map_err(|a| Error::DatabaseError(format!("Error connecting to {}", a.to_string())))
}

pub fn get_device(conn: &PgConnection, d_id: i32) -> Result<Device, Error> {
    use super::schema::devices::dsl::*;
    devices
        .find(d_id)
        .first::<Device>(conn)
        .map_err(|e| Error::DatabaseError(format!("Error get device to {}", e.to_string())))
}

pub fn get_id_by_deviceid(conn: &PgConnection, d_id: String) -> Result<i32,Error> {
    use super::schema::devices::dsl::*;
    devices
        .select(id)
        .filter(device_id.eq(d_id))
        .first::<i32>(conn)
        .map_err(|e| Error::DatabaseError(format!("Error get device to {}", e.to_string())))
}

pub fn topics() -> Result<Vec<String>,Error> {
    use super::schema::devices::dsl::*;
    let conn = db_connection()?;
    devices.select(device_id)
        .load::<String>(&conn)
        .map(|result|result.iter().map(|topic|format!("$USR/DevJsonTx/{}",topic)).collect())
        .map_err(|e|Error::DatabaseError(e.to_string()))
        
}

pub fn new_rainfall(
    device:i32,
    data:f32,
    
) -> Result<usize,Error> {
    use super::schema::rainfalls::dsl::*;
    let conn = db_connection()?;
    
    let rainfall = NewRainfall::new(device, data);
    diesel::insert_into(rainfalls)
    .values(&rainfall)
    .execute(&conn)
    .map_err(|a| {
        Error::DatabaseError(format!("Error create rainfalls to {}", a.to_string()))
    })
}

pub fn new_water_depth(device:i32,data:f32) -> Result<usize,Error> {
    use super::schema::water_depths::dsl::*;
    let conn = db_connection()?;
    let f_value = cal_flow_value(&conn, device, data)?;
    let water_depth = NewWaterDepth::new(device, data,f_value);
    diesel::insert_into(water_depths)
    .values(&water_depth)
    .execute(&conn)
    .map_err(|a| {
        Error::DatabaseError(format!("Error create water_depth to {}", a.to_string()))
    })
}

fn cal_flow_value(conn:&PgConnection,device:i32,data:f32) -> Result<f32,Error> {
    let device = get_device(conn, device)?;
    let m = 1.5;
    let b = device.stream_width.map(|w|decimal_to_f32(&w)).unwrap_or(0.0);
    
    let f_value = m*b*data.powf(1.5);
    Ok(f_value)
}

pub fn new_calculation(dev_id:i32,rain:f32) -> Result<usize,Error> {
    use super::schema::calculations::dsl::*;
    let conn = db_connection()?;
    let mut si_1 = -30.0;let mut wi_1 = 0.0;let mut interval=300.0;
    let recent_cal = recent_calculation(&conn, dev_id);
    if let Ok(rc) = recent_cal {
        si_1 = decimal_to_f32(&rc.storage);
        wi_1 = decimal_to_f32(&rc.wi);
        let now_stamps = Utc::now().timestamp();
        let recent_stamps = rc.create_time.timestamp();
        interval = (now_stamps - recent_stamps).to_f32().unwrap_or(300.0);    
        
    }
    let (i_j,s_i) = cal_storage(si_1, rain, interval);
    let (qu,w_i) = cal_quantity(i_j, wi_1, interval);
    let calculation = NewCalculation::new(dev_id, s_i, w_i, qu);
    
    diesel::insert_into(calculations)
    .values(&calculation)
    .execute(&conn)
    .map_err(|a| {
        Error::DatabaseError(format!("Error create calculation to {}", a.to_string()))
    })
    
}
// I净，s_i
fn cal_storage(si_1:f32,rain:f32,interval:f32) -> (f32,f32){
    let si = si_1+rain - 3.0*interval/24.0/12.0/300.0;
    let si = si.max(-30.0);
    if si > 0.0 {
        (si,0.0)
    } else {
        (0.0,si)
    }
}

// quantity,wi
fn cal_quantity(i_j:f32,wi_1:f32,interval:f32) -> (f32,f32) {
    let f= 55.0;let a = 0.00045;let b = 0.7;
    let iil = i_j*f*3.333;
    let d = b/(2.0*a);
    let g = (b*b+4.0*a*iil).powf(0.5)/(2.0*a);    
    let k = (wi_1 +d - g)/(wi_1+d+g)*(-2.0*g*interval/10000.0).exp();
    let wi = (1.0+k)/(1.0-k)*g -d;
    let qi = a*wi*wi+b*wi;
    (qi,wi)
}

pub fn recent_calculation(conn:&PgConnection,dev_id:i32) -> Result<Calculation,Error> {
    use super::schema::*;

    calculations::table
        .find(dev_id)
        .order_by(calculations::create_time)
        .first::<Calculation>(conn)
        .map_err(|a| {
            Error::DatabaseError(format!("Error get calculation to {}", a.to_string()))
        })


}
fn six_timestamps() -> (i64,i64,i64,i64,i64,i64) {
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps/1800;
    let one_stamps = now_stamps/3600;
    let one_half_stamps = now_stamps/5400;
    let two_stamps = now_stamps/7200;
    let three_stamps = now_stamps/10800;
    (now_stamps,half_stamps,one_stamps,one_half_stamps,two_stamps,three_stamps)
}

pub fn device_names(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<(String,String)>,Error> {
    use super::schema::*;       
    
    devices::table                        
        .select((devices::name,devices::region))        
        .order_by(devices::id)        
        .filter(devices::id.eq_any(dev_ids))        
        .load::<(String,String)>(conn)
        .map_err(|a| {
            Error::DatabaseError(format!("Error get rainfalls title by three hours to {}", a.to_string()))
        })   
    
}

pub fn all_devices(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<Device>,Error> {
    use super::schema::*;
    
    devices::table
        .order_by(devices::id)
        .filter(devices::id.eq_any(dev_ids))
        .load::<Device>(conn)
        .map_err(|a| {
            Error::DatabaseError(format!("Error get rainfalls title by three hours to {}", a.to_string()))
        }) 

}

pub fn device_ids(conn:&PgConnection) -> Result<Vec<i32>,Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps/1800*1800;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);
    rainfalls::table                        
        .select(rainfalls::device_id)
        .group_by(rainfalls::device_id)
        .order_by(rainfalls::device_id)        
        .filter(rainfalls::create_time.ge(n_time))        
        .load::<i32>(conn)
        .map_err(|a| {
            Error::DatabaseError(format!("Error get rainfalls title by three hours to {}", a.to_string()))
        })  
}

pub fn rainfall_by_half(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<Option<f32>>,Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps/1800*1800;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let value_sum = rainfalls::table                
        .select(sum(rainfalls::value))
        .group_by(rainfalls::device_id)
        .order_by(rainfalls::device_id)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq_any(dev_ids)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;    

    let data:Vec<_> = value_sum.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok(data)
}

pub fn flow_by_half(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<Option<f32>>,Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps/1800*1800;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let flow_avg = water_depths::table
        .group_by(water_depths::device_id)
        .order_by(water_depths::device_id)        
        .filter(water_depths::create_time.ge(n_time).and(water_depths::device_id.eq_any(dev_ids)))
        .select(avg(water_depths::flow_value))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;    

    let data:Vec<_> = flow_avg.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok(data)
}

pub fn a_flows_by_half(conn:&PgConnection,dev_id:i32) -> Result<(Vec<Option<f32>>,Vec<String>),Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps-8*3600;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let flow_avg = water_depths::table                
        .select(avg(water_depths::flow_value))
        .group_by(water_depths::half_hour)
        .order_by(water_depths::half_hour)        
        .filter(water_depths::create_time.ge(n_time).and(water_depths::device_id.eq(dev_id)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?; 
    
    let times = water_depths::table                
        .select(water_depths::half_hour)
        .group_by(water_depths::half_hour)
        .order_by(water_depths::half_hour)        
        .filter(water_depths::create_time.ge(n_time).and(water_depths::device_id.eq(dev_id)))        
        .load::<Option<i64>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;

    let times:Vec<_> = times.iter().map(|t|NaiveDateTime::from_timestamp(t.unwrap_or(0)*1800+8*3600, 0).time().to_string()).collect();
    let flow_avg:Vec<_> = flow_avg.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok((flow_avg,times))
}

pub fn depth_by_half(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<Option<f32>>,Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps/1800*1800;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let depth_avg = water_depths::table
        .group_by(water_depths::device_id)
        .order_by(water_depths::device_id)        
        .filter(water_depths::create_time.ge(n_time).and(water_depths::device_id.eq_any(dev_ids)))
        .select(avg(water_depths::value))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;    

    let data:Vec<_> = depth_avg.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok(data)
}

pub fn a_depths_by_half(conn:&PgConnection,dev_id:i32) -> Result<(Vec<Option<f32>>,Vec<String>),Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps-8*3600;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let depth_avg = water_depths::table                
        .select(avg(water_depths::value))
        .group_by(water_depths::half_hour)
        .order_by(water_depths::half_hour)        
        .filter(water_depths::create_time.ge(n_time).and(water_depths::device_id.eq(dev_id)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?; 
    
    let times = water_depths::table                
        .select(water_depths::half_hour)
        .group_by(water_depths::half_hour)
        .order_by(water_depths::half_hour)        
        .filter(water_depths::create_time.ge(n_time).and(water_depths::device_id.eq(dev_id)))        
        .load::<Option<i64>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;

    let times:Vec<_> = times.iter().map(|t|NaiveDateTime::from_timestamp(t.unwrap_or(0)*1800+8*3600, 0).time().to_string()).collect();
    let depth_avg:Vec<_> = depth_avg.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok((depth_avg,times))
}

pub fn quantity_by_half(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<Option<f32>>,Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps/1800*1800;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let quantity_avg = calculations::table
        .group_by(calculations::device_id)
        .order_by(calculations::device_id)        
        .filter(calculations::create_time.ge(n_time).and(calculations::device_id.eq_any(dev_ids)))
        .select(avg(calculations::quantity))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;    

    let data:Vec<_> = quantity_avg.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok(data)
}

pub fn a_quantitys_by_half(conn:&PgConnection,dev_id:i32) -> Result<(Vec<Option<f32>>,Vec<String>),Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps-8*3600;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let quantity_avg = calculations::table                
        .select(avg(calculations::quantity))
        .group_by(calculations::half_hour)
        .order_by(calculations::half_hour)        
        .filter(calculations::create_time.ge(n_time).and(calculations::device_id.eq(dev_id)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?; 
    
    let times = calculations::table                
        .select(calculations::half_hour)
        .group_by(calculations::half_hour)
        .order_by(calculations::half_hour)        
        .filter(calculations::create_time.ge(n_time).and(calculations::device_id.eq(dev_id)))        
        .load::<i64>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;

    let times:Vec<_> = times.iter().map(|t|NaiveDateTime::from_timestamp(t*1800+8*3600, 0).time().to_string()).collect();
    let quantity_avg:Vec<_> = quantity_avg.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok((quantity_avg,times))
}

pub fn a_rainfalls_by_half(conn:&PgConnection,dev_id:i32) -> Result<(Vec<Option<f32>>,Vec<String>),Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps-8*3600;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let value_sum = rainfalls::table                
        .select(sum(rainfalls::value))
        .group_by(rainfalls::half_hour)
        .order_by(rainfalls::half_hour)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq(dev_id)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?; 
    
    let times = rainfalls::table                
        .select(rainfalls::half_hour)
        .group_by(rainfalls::half_hour)
        .order_by(rainfalls::half_hour)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq(dev_id)))        
        .load::<i64>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;

    let times:Vec<_> = times.iter().map(|t|NaiveDateTime::from_timestamp(t*1800+8*3600, 0).time().to_string()).collect();
    let value_sum:Vec<_> = value_sum.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok((value_sum,times))
}

pub fn rainfall_by_one(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<Option<f32>>,Error> {
    use super::schema::*;    
    let now_stamps = Utc::now().timestamp();
    let one_stamps = now_stamps/3600*3600;
    let n_time = NaiveDateTime::from_timestamp(one_stamps, 0);    
    
        
    let value_sum = rainfalls::table                
        .select(sum(rainfalls::value))
        .group_by(rainfalls::device_id)
        .order_by(rainfalls::device_id)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq_any(dev_ids)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by one hours to {}", a.to_string()))
    })?;    

    let data:Vec<_> = value_sum.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok(data)
}

pub fn a_rainfalls_by_one(conn:&PgConnection,dev_id:i32) -> Result<(Vec<Option<f32>>,Vec<String>),Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps-12*3600;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let value_sum = rainfalls::table                
        .select(sum(rainfalls::value))
        .group_by(rainfalls::one_hour)
        .order_by(rainfalls::one_hour)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq(dev_id)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?; 
    
    let times = rainfalls::table                
        .select(rainfalls::one_hour)
        .group_by(rainfalls::one_hour)
        .order_by(rainfalls::one_hour)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq(dev_id)))        
        .load::<i64>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;

    let times:Vec<_> = times.iter().map(|t|NaiveDateTime::from_timestamp(t*3600+8*3600, 0).time().to_string()).collect();
    let value_sum:Vec<_> = value_sum.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok((value_sum,times))
}

pub fn rainfall_by_one_half(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<Option<f32>>,Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let one_half_stamps = now_stamps/5400*5400;
    let n_time = NaiveDateTime::from_timestamp(one_half_stamps, 0);
    
    
        
    let value_sum = rainfalls::table                
        .select(sum(rainfalls::value))
        .group_by(rainfalls::device_id)
        .order_by(rainfalls::device_id)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq_any(dev_ids)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by one hours to {}", a.to_string()))
    })?;    

    let data:Vec<_> = value_sum.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok(data)
}

pub fn a_rainfalls_by_one_half(conn:&PgConnection,dev_id:i32) -> Result<(Vec<Option<f32>>,Vec<String>),Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps-12*3600;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let value_sum = rainfalls::table                
        .select(sum(rainfalls::value))
        .group_by(rainfalls::one_half_hour)
        .order_by(rainfalls::one_half_hour)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq(dev_id)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?; 
    
    let times = rainfalls::table                
        .select(rainfalls::one_half_hour)
        .group_by(rainfalls::one_half_hour)
        .order_by(rainfalls::one_half_hour)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq(dev_id)))        
        .load::<i64>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;

    let times:Vec<_> = times.iter().map(|t|NaiveDateTime::from_timestamp(t*5400+8*3600, 0).time().to_string()).collect();
    let value_sum:Vec<_> = value_sum.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok((value_sum,times))
}

pub fn rainfall_by_two(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<Option<f32>>,Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let two_stamps = now_stamps/7200*7200;
    let n_time = NaiveDateTime::from_timestamp(two_stamps, 0);
    
        
    let value_sum = rainfalls::table                
        .select(sum(rainfalls::value))
        .group_by(rainfalls::device_id)
        .order_by(rainfalls::device_id)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq_any(dev_ids)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by one hours to {}", a.to_string()))
    })?;    

    let data:Vec<_> = value_sum.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok(data)
}

pub fn a_rainfalls_by_two(conn:&PgConnection,dev_id:i32) -> Result<(Vec<Option<f32>>,Vec<String>),Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps-16*3600;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let value_sum = rainfalls::table                
        .select(sum(rainfalls::value))
        .group_by(rainfalls::two_hour)
        .order_by(rainfalls::two_hour)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq(dev_id)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?; 
    
    let times = rainfalls::table                
        .select(rainfalls::two_hour)
        .group_by(rainfalls::two_hour)
        .order_by(rainfalls::two_hour)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq(dev_id)))        
        .load::<i64>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;

    let times:Vec<_> = times.iter().map(|t|NaiveDateTime::from_timestamp(t*7200+8*3600, 0).time().to_string()).collect();
    let value_sum:Vec<_> = value_sum.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok((value_sum,times))
}

pub fn rainfall_by_three(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<Option<f32>>,Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let three_stamps = now_stamps/10800*10800;
    let n_time = NaiveDateTime::from_timestamp(three_stamps, 0);
    
        
    let value_sum = rainfalls::table                
        .select(sum(rainfalls::value))
        .group_by(rainfalls::device_id)
        .order_by(rainfalls::device_id)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq_any(dev_ids)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by one hours to {}", a.to_string()))
    })?;    

    let data:Vec<_> = value_sum.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok(data)
}

pub fn a_rainfalls_by_three(conn:&PgConnection,dev_id:i32) -> Result<(Vec<Option<f32>>,Vec<String>),Error> {
    use super::schema::*;
    let now_stamps = Utc::now().timestamp();
    let half_stamps = now_stamps-24*3600;
    let n_time = NaiveDateTime::from_timestamp(half_stamps, 0);    
    
        
    let value_sum = rainfalls::table                
        .select(sum(rainfalls::value))
        .group_by(rainfalls::three_hour)
        .order_by(rainfalls::three_hour)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq(dev_id)))        
        .load::<Option<BigDecimal>>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?; 
    
    let times = rainfalls::table                
        .select(rainfalls::three_hour)
        .group_by(rainfalls::three_hour)
        .order_by(rainfalls::three_hour)        
        .filter(rainfalls::create_time.ge(n_time).and(rainfalls::device_id.eq(dev_id)))        
        .load::<i64>(conn)
        .map_err(|a| {
        Error::DatabaseError(format!("Error get rainfalls sum value by half hours to {}", a.to_string()))
    })?;

    let times:Vec<_> = times.iter().map(|t|NaiveDateTime::from_timestamp(t*10800+8*3600, 0).time().to_string()).collect();
    let value_sum:Vec<_> = value_sum.iter().map(|v|v.as_ref().map(|v1|decimal_to_f32(&v1))).collect();
    
    Ok((value_sum,times))
}

fn decimal_to_f32(v:&BigDecimal) -> f32 {
    v.with_scale(2).to_f32().unwrap_or(0.0)
}

pub fn water_depth_of_recently(conn:&PgConnection,dev_ids:&Vec<i32>) -> Result<Vec<f32>,Error> {
    use super::schema::*;
    let mut datas:Vec<f32> = vec![];
    for dev_id in dev_ids {
        let data=water_depths::table
            .select(water_depths::value)
            .filter(water_depths::device_id.eq(dev_id))
            .order_by(water_depths::create_time.desc())
            .first::<BigDecimal>(conn)
            .map_err(|a| {
                Error::DatabaseError(format!("Error get water depth value by one hours to {}", a.to_string()))
            })?;
        let data = decimal_to_f32(&data);
        datas.push(data);
    }
    Ok(datas)
}

pub fn update_devices_swidth(conn:&PgConnection) {
    use super::schema::devices::dsl::*;
    let names = vec!["上半岙","陈家岙","李家","半岙","相岙","阮家","五联村",
        "黄土岭","杜徐岙","翁岙","孔岙","余鲍陈","西岙","南雷村",
        "金岙","向家弄上游","向家弄下游","冯村","横坎头村","湖东村","石潭",
        "中姚","上庄","大俞","黑龙潭","丁家畈","梨洲村","棠溪","吊钩星",
        "茶培","北溪","芝林","大隐村",
    ];
    let widths = vec![5,7,12,5,12,3,8,5,5,5,5,2,14,11,8,6,14,4,11,10,11,10,
        11,20,10,3,6,7,10,7,35,24,46
    ];
    for i in 0..names.len() {
        
        let target = devices.filter(name.eq(names[i]));
        let wd = BigDecimal::from_i64(widths[i]);
        diesel::update(target).set(stream_width.eq(wd)).execute(conn).unwrap();
        
    }

}
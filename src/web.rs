use rocket_contrib::databases::diesel::PgConnection;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use serde::Serialize;
use super::error::Error;
use super::models;


#[database("mountain_torrents")]
pub struct DbConn(PgConnection);

pub fn run() {
    rocket::ignite()    
        .attach(DbConn::fairing())
        .mount("/api", routes![
            mt_current,
            half_rain,
            half_depth,
            half_flow,
            half_quantity,
            one_rain,
            one_half_rain,
            two_rain,
            three_rain,
        ])
        .mount("/", StaticFiles::from("dist"))        
        .launch();
}

#[derive(Serialize)]
pub struct MTRow {
    id:i32,
    name:String,
    region:String,
    depth:f32,
    depth_def:f32,
    half_rain:f32,
    half_rain_def:f32,
    one_rain:f32,
    one_rain_def:f32,
    one_half_rain:f32,
    one_half_rain_def:f32,
    two_rain:f32,
    two_rain_def:f32,
    three_rain:f32,
    three_rain_def:f32,
    quantity:f32,
    flow:f32,
}

#[derive(Serialize)]
pub struct ChartData {
    values:Vec<Option<f32>>,
    times:Vec<String>,
    describe:String,
}

#[get("/mt_current")]
pub fn mt_current(conn:DbConn) -> Result<Json<Vec<MTRow>>,Error> {
    let dev_ids = models::device_ids(&conn)?;
    let devs = models::all_devices(&conn,&dev_ids)?;
    let depths = models::water_depth_of_recently(&conn, &dev_ids)?;
    let half_rains = models::rainfall_by_half(&conn, &dev_ids)?;
    let one_rains = models::rainfall_by_one(&conn, &dev_ids)?;
    let one_half_rains = models::rainfall_by_one_half(&conn, &dev_ids)?;
    let two_rains = models::rainfall_by_two(&conn, &dev_ids)?;
    let three_rains = models::rainfall_by_three(&conn, &dev_ids)?;
    let quantitys = models::quantity_by_half(&conn, &dev_ids)?;
    let flows = models::flow_by_half(&conn, &dev_ids)?;

    let mut mts:Vec<MTRow> = vec![];
    for i in 0..dev_ids.len() {
        let mt = MTRow {
            id:dev_ids[i],
            name:devs[i].name.clone(),
            region:devs[i].region.clone(),
            depth:depths[i],
            depth_def:devs[i].height_def(),
            half_rain:half_rains[i].unwrap_or(0.0),
            half_rain_def:devs[i].half_hour_def(),
            one_rain:one_rains[i].unwrap_or(0.0),
            one_rain_def:devs[i].one_hour_def(),
            one_half_rain:one_half_rains[i].unwrap_or(0.0),
            one_half_rain_def:devs[i].one_half_hour_def(),
            two_rain:two_rains[i].unwrap_or(0.0),
            two_rain_def:devs[i].two_hour_def(),
            three_rain:three_rains[i].unwrap_or(0.0),
            three_rain_def:devs[i].three_hour_def(),
            quantity:quantitys[i].unwrap_or(0.0),
            flow:flows[i].unwrap_or(0.0),
        };
        mts.push(mt);
    }
    Ok(Json(mts))
}
#[get("/half_rain?<dev_id>")]
pub fn half_rain(conn:DbConn,dev_id:i32) -> Result<Json<ChartData>,Error> {
    
    let half_rains = models::a_rainfalls_by_half(&conn, dev_id)?;
    
    let data = ChartData {
        values:half_rains.0,
        times:half_rains.1,
        describe:"0.5小时雨量".to_string(),
    };
    
    Ok(Json(data))
}

#[get("/half_depth?<dev_id>")]
pub fn half_depth(conn:DbConn,dev_id:i32) -> Result<Json<ChartData>,Error> {
    
    let half_depths = models::a_depths_by_half(&conn, dev_id)?;
    
    let data = ChartData {
        values:half_depths.0,
        times:half_depths.1,
        describe:"水深".to_string(),
    };
    
    Ok(Json(data))
}

#[get("/half_flow?<dev_id>")]
pub fn half_flow(conn:DbConn,dev_id:i32) -> Result<Json<ChartData>,Error> {
    
    let half_flows = models::a_flows_by_half(&conn, dev_id)?;
    
    let data = ChartData {
        values:half_flows.0,
        times:half_flows.1,
        describe:"流量".to_string(),
    };
    
    Ok(Json(data))
}

#[get("/half_quantity?<dev_id>")]
pub fn half_quantity(conn:DbConn,dev_id:i32) -> Result<Json<ChartData>,Error> {
    
    let half_quantitys = models::a_quantitys_by_half(&conn, dev_id)?;
    
    let data = ChartData {
        values:half_quantitys.0,
        times:half_quantitys.1,
        describe:"计算流量".to_string(),
    };
    
    Ok(Json(data))
}

#[get("/one_rain?<dev_id>")]
pub fn one_rain(conn:DbConn,dev_id:i32) -> Result<Json<ChartData>,Error> {
    
    let one_rains = models::a_rainfalls_by_one(&conn, dev_id)?;
    
    let data = ChartData {
        values:one_rains.0,
        times:one_rains.1,
        describe:"1小时雨量".to_string(),
    };
    
    Ok(Json(data))
}
#[get("/one_half_rain?<dev_id>")]
pub fn one_half_rain(conn:DbConn,dev_id:i32) -> Result<Json<ChartData>,Error> {
    
    let one_half_rains = models::a_rainfalls_by_one_half(&conn, dev_id)?;
    
    let data = ChartData {
        values:one_half_rains.0,
        times:one_half_rains.1,
        describe:"1.5小时雨量".to_string(),
    };
    
    Ok(Json(data))
}
#[get("/two_rain?<dev_id>")]
pub fn two_rain(conn:DbConn,dev_id:i32) -> Result<Json<ChartData>,Error> {
    
    let two_rains = models::a_rainfalls_by_two(&conn, dev_id)?;
    
    let data = ChartData {
        values:two_rains.0,
        times:two_rains.1,
        describe:"2小时雨量".to_string(),
    };
    
    Ok(Json(data))
}
#[get("/three_rain?<dev_id>")]
pub fn three_rain(conn:DbConn,dev_id:i32) -> Result<Json<ChartData>,Error> {
    
    let three_rains = models::a_rainfalls_by_three(&conn, dev_id)?;
    
    let data = ChartData {
        values:three_rains.0,
        times:three_rains.1,
        describe:"3小时雨量".to_string(),
    };
    
    Ok(Json(data))
}
use mountain_torrents::models::*;
fn main() {
    let conn = db_connection().unwrap();
    update_devices_swidth(&conn);

}

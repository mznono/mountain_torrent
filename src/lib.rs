#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate rocket_contrib;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



pub mod schema;
pub mod models;
pub mod error;
pub mod mqtt_client;
pub mod sum;
pub mod web;


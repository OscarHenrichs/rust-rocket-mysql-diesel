#[macro_use] 
extern crate rocket;
extern crate diesel;
extern crate dotenv;


mod db;
mod controller;
mod model;
mod routes;
mod constants;


use routes::test_routes::{ get_hellow, get_hellow_sleep };
use constants::catchers::{ not_found };


#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    rocket::build().register("/", catchers![not_found]).manage(db::connect()).mount("/", routes![get_hellow, get_hellow_sleep])
}

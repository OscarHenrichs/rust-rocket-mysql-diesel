use serde::Deserialize;
use rocket::serde::json::{Json, Value, json};
use rocket::tokio::time::{sleep, Duration};

#[derive(Deserialize)]
pub struct ProductQuery {
    price_from: f32,
    price_to: f32,
}

#[get("/")]
pub async fn get_hellow() -> Value {
    json!("Hello, world!")
}

#[get("/hello/<seconds>")]
pub async fn get_hellow_sleep(seconds: u64) -> Value {
    sleep(Duration::from_secs(seconds)).await;
    json!("Hello, world!")
}
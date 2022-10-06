use serde::Deserialize;
use rocket::serde::json::{Json, Value, json};

#[catch(404)]
pub async fn not_found() -> Value {
    json!("Not found!")
}
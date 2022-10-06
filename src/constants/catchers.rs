use rocket::serde::json::{Json, Value, json};

#[catch(404)]
pub async fn not_found() -> Json<Value> {
    json!("Not found!")
}
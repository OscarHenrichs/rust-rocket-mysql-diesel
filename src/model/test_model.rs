use serde::{Serialize, Deserialize};
 
#[derive(Serialize, Deserialize)]
pub struct Product {
    pub id: u64,
    pub code: String,
    pub product_name: String
}
use rocket::post;
use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LavadoData {
    pub start: String,
    pub destination: String,
    pub transactions: Vec<String>,
}

#[post("/lavado", format = "json", data = "<data>")]
pub fn post_lavado(data: Json<LavadoData>) -> String {
    format!(
        "Analizando ruta desde {} hasta {}. Total transacciones: {}",
        data.start,
        data.destination,
        data.transactions.len()
    )
}

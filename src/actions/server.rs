use crate::routes::lavado;
use rocket::fs::{FileServer, relative};

#[rocket::get("/")]
async fn index() -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open(relative!("frontend/dist/index.html"))
        .await
        .ok()
}

pub fn init_rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount("/", rocket::routes![index])
        .mount("/", FileServer::from(relative!("frontend/dist/")))
        .mount("/api", rocket::routes![lavado::post_lavado])
}

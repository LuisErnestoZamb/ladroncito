use ladroncito::actions::server;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _ = server::init_rocket().launch().await?;

    Ok(())
}

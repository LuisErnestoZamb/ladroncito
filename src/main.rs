use clap::{Arg, Command};
use ladroncito::actions::processing::process_graph;
use ladroncito::actions::server;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let matches = Command::new("ladroncito")
        .about("Blockchain Transaction Forensics Tool")
        .subcommand(Command::new("web").about("Start the Rocket web server"))
        .get_matches();

    match matches.subcommand() {
        Some(("web", _)) => {
            println!("Starting web server...");
            server::init_rocket().launch().await?;
        }
        _ => {
            println!("Initializing local graph analysis...");
            process_graph();
        }
    }

    Ok(())
}

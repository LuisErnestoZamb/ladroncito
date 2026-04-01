use clap::Command;
use ladroncito::actions::one_path::process_one_graph;
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
        Some(("core", _)) => {
            println!(
                "Running in multithread mode. Be aware that you computer can become frozen. Use this carefully."
            );
            process_graph("core");
        }
        Some(("parallel", _)) => {
            println!(
                "Running in parallel mode. Be aware that you computer can become frozen. Use this carefully."
            );
            process_graph("parallel");
        }
        Some(("all", _)) => {
            println!("Search all occurrences, not optimized.");
            process_graph("all");
        }
        _ => {
            println!("Initializing local graph analysis...");
            process_one_graph();
        }
    }

    Ok(())
}

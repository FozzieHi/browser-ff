use clap::App;

mod commands;

use commands::{download, rebuild};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new("melon")
        .version(std::env!("CARGO_PKG_VERSION"))
        .subcommand(
            App::new("download")
                .about("Downloads source code")
        )
        .subcommand(
            App::new("rebuild")
                .about("Rebuild melon tool")
        );

    let matches = app.clone().get_matches();

    match matches.subcommand_name() {
        Some("download") => download()?,
        Some("rebuild") => rebuild()?,

        Some(_) => {
            println!("Unknown command. Use --help for a list of commands.");
        }

        None => {
            app.print_help().unwrap();
            println!();
        }
    }

    Ok(())
}

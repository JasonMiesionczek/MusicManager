use clap::{App, Arg, SubCommand};
use cli::commands::{db::run_migrations, music::search_command};

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let matches = App::new("music-manager")
        .subcommands(vec![
            SubCommand::with_name("music") // The name we call argument with
                .about("Commands for managing music") // The message displayed in "myapp -h"
                // or "myapp help"
                .subcommand(
                    SubCommand::with_name("search")
                        .about("Search for an artist")
                        .arg(
                            Arg::with_name("artist_name") // And their own arguments
                                .help("Artist to search for")
                                .index(1)
                                .required(true),
                        ),
                ),
            SubCommand::with_name("db")
                .about("Commands for working with the database")
                .subcommands(vec![
                    SubCommand::with_name("migrate").about("Work with database migrations")
                ]),
        ])
        .get_matches();

    match matches.subcommand() {
        ("music", Some(music_matches)) => match music_matches.subcommand() {
            ("search", Some(search_matches)) => {
                search_command(search_matches.value_of("artist_name").unwrap())
            }
            ("", None) => println!("No subcommand was used"),
            _ => println!("invalid subcommand"),
        },
        ("db", Some(db_matches)) => match db_matches.subcommand() {
            ("migrate", Some(_)) => run_migrations(),
            ("", None) => println!("No subcommand specified"),
            _ => println!("invalud subcommand"),
        },
        ("", None) => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }
}

#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use std::fs::File;

mod database;

fn main() {
    let matches = App::new("SSH Permit A38")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("database")
                .value_name("FILE")
                .help("Sets a database^ file")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("server")
                .about("controls testing features")
                .subcommand(
                    SubCommand::with_name("add").arg(
                        Arg::with_name("debug")
                            .short("d")
                            .help("print debug information verbosely"),
                    ),
                ),
        )
        .get_matches();

    let database_file = matches.value_of("database").unwrap_or("ssh-permit.json");
    println!("Value for config: {}", database_file);
    let user = database::User {
        name: "a".to_owned(),
        public_key: "b".to_owned(),
    };

    let hostname = "urlsmash.403.io";

    let database = database::Database {
        hosts: vec![
            database::Host {
                hostname: hostname.to_owned(),
                authorized: vec![],
            },
        ],
        users: vec![user],
        ..Default::default()
    };

    let file = File::create(database_file).unwrap();
    serde_json::to_writer_pretty(&file, &database).expect("Unable to write database file.");

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    // more program logic goes here...
}

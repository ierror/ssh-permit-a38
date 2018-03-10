extern crate chrono;
extern crate colored;
extern crate serde;
extern crate serde_json;
extern crate ssh2;

#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate difference;
extern crate term;

use clap::{App, Arg, SubCommand};

mod cli_flow;
mod database;
mod subcommand_host;
mod subcommand_sync;
mod subcommand_user;

fn main() {
    let matches = App::new("SSH Permit A38")
        // application info
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        // --database
        .arg(
            Arg::with_name("database")
                .short("d")
                .long("database")
                .value_name("FILE")
                .help("Database file to use")
                .takes_value(true),
        )
        // host
        .subcommand(
            SubCommand::with_name("host")
                // host <host>
                .about("Host related actions")
                        .arg(Arg::with_name("host:port")
                            .help("Host")
                            .index(1))
                // host <host> add
                .subcommand(
                    SubCommand::with_name("add")
                )
                // host <host> remove
                .subcommand(
                    SubCommand::with_name("remove")
                )
                // host list
                .subcommand(
                    SubCommand::with_name("list")
                )
        )
        // user
        .subcommand(
            SubCommand::with_name("user")
                // user <host>
                .about("User related actions")
                        .arg(Arg::with_name("user")
                            .help("User")
                            .index(1))
                // user <user> add
                .subcommand(
                    SubCommand::with_name("add")
                )
                // user <user> remove
                .subcommand(
                    SubCommand::with_name("remove")
                )
                // user list
                .subcommand(
                    SubCommand::with_name("list")
                )
                // user <user> grant <host>
                .subcommand(
                    SubCommand::with_name("grant")
                        .arg(Arg::with_name("host")
                            .help("Host")
                            .index(1)
                            .required(true))
                )
                // user <user> grant <host>
                .subcommand(
                    SubCommand::with_name("revoke")
                        .arg(Arg::with_name("host")
                            .help("Host")
                            .index(1)
                            .required(true))
                )
        )
        // sync
        .subcommand(
            SubCommand::with_name("sync")
                // Sync
                .about("Sync pending changes to the related hosts")
        )
        .get_matches();

    let database_file = matches.value_of("database").unwrap_or("ssh-permit.json");
    println!("Value for config: {}", database_file);

    let mut db = database::Database {
        ..Default::default()
    }.load(database_file)
        .unwrap();

    // host
    if let Some(matches) = matches.subcommand_matches("host") {
        let hostname = matches.value_of("host:port").unwrap_or("");

        if matches.subcommand_matches("add").is_some() {
            subcommand_host::add(&mut db, &hostname);
        } else if matches.subcommand_matches("remove").is_some() {
            subcommand_host::remove(&mut db, &hostname);
        } else if matches.subcommand_matches("list").is_some() {
            subcommand_host::list(&mut db, &hostname);
        }
    }

    // user
    if let Some(matches) = matches.subcommand_matches("user") {
        let user_id = matches.value_of("user").unwrap_or("");

        if matches.subcommand_matches("add").is_some() {
            subcommand_user::add(&mut db, &user_id);
        } else if matches.subcommand_matches("remove").is_some() {
            subcommand_user::remove(&mut db, &user_id);
        } else if let Some(_matches) = matches.subcommand_matches("list") {
            subcommand_user::list(&mut db);
        } else if let Some(matches) = matches.subcommand_matches("grant") {
            let hostname = matches.value_of("host").unwrap();
            subcommand_user::grant(&mut db, &user_id, &hostname);
        } else if let Some(matches) = matches.subcommand_matches("revoke") {
            let hostname = matches.value_of("host").unwrap();
            subcommand_user::revoke(&mut db, &user_id, &hostname);
        }
    } else if let Some(matches) = matches.subcommand_matches("sync") {
        subcommand_sync::sync(&mut db);
    }

    db.save(&database_file);
}

extern crate chrono;
extern crate colored;
extern crate serde;
extern crate serde_json;
extern crate ssh2;

#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate difference;

use clap::{App, Arg, SubCommand};

mod cli_flow;
mod database;
mod subcommand_group;
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

        // group
        .subcommand(
            SubCommand::with_name("group")
                // group <host>
                .about("Group related actions")
                        .arg(Arg::with_name("group")
                            .help("Group")
                            .index(1))
                // group <group> add
                .subcommand(
                    SubCommand::with_name("add")
                )
                // group <group> remove
                .subcommand(
                    SubCommand::with_name("remove")
                )
                // group list
                .subcommand(
                    SubCommand::with_name("list")
                )
                // group <group> grant <host>
                .subcommand(
                    SubCommand::with_name("grant")
                        .arg(Arg::with_name("host")
                            .help("Host")
                            .index(1)
                            .required(true))
                )
                // group <group> revoke <host>
                .subcommand(
                    SubCommand::with_name("revoke")
                        .arg(Arg::with_name("host")
                            .help("Host")
                            .index(1)
                            .required(true))
                )
                // group <group> user <user>
                .subcommand(
                    SubCommand::with_name("user")
                        .arg(Arg::with_name("user")
                            .help("User")
                            .index(1)
                            .required(true))
                        .subcommand(
                            SubCommand::with_name("add")
                        )
                        .subcommand(
                            SubCommand::with_name("remove")
                        )
                )
        )

        // sync
        .subcommand(
            SubCommand::with_name("sync")
                // Sync
                .about("Sync pending changes to the related hosts")
        )
        .get_matches();

    // load database
    let database_file = matches.value_of("database").unwrap_or("ssh-permit.json");

    let mut db = database::Database {
        ..Default::default()
    };

    db = match db.load(database_file) {
        Ok(t) => t,
        Err(e) => {
            cli_flow::errorln(&format!(
                "Unable to load {}: {}",
                database_file,
                e.to_string()
            ));
            return;
        }
    };

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
    else if let Some(matches) = matches.subcommand_matches("user") {
        let user_id = matches.value_of("user").unwrap_or("");

        if matches.subcommand_matches("add").is_some() {
            subcommand_user::add(&mut db, &user_id);
        } else if matches.subcommand_matches("remove").is_some() {
            subcommand_user::remove(&mut db, &user_id);
        } else if let Some(_matches) = matches.subcommand_matches("list") {
            subcommand_user::list(&mut db, &user_id);
        } else if let Some(matches) = matches.subcommand_matches("grant") {
            let hostname = matches.value_of("host").unwrap();
            subcommand_user::grant(&mut db, &user_id, &hostname);
        } else if let Some(matches) = matches.subcommand_matches("revoke") {
            let hostname = matches.value_of("host").unwrap();
            subcommand_user::revoke(&mut db, &user_id, &hostname);
        }
    }
    // group
    else if let Some(matches) = matches.subcommand_matches("group") {
        let group_id = matches.value_of("group").unwrap_or("");

        if matches.subcommand_matches("add").is_some() {
            subcommand_group::add(&mut db, &group_id);
        } else if matches.subcommand_matches("remove").is_some() {
            subcommand_group::remove(&mut db, &group_id);
        } else if let Some(_matches) = matches.subcommand_matches("list") {
            subcommand_group::list(&mut db);
        } else if let Some(matches) = matches.subcommand_matches("grant") {
            let hostname = matches.value_of("host").unwrap();
            subcommand_group::grant(&mut db, &group_id, &hostname);
        } else if let Some(matches) = matches.subcommand_matches("revoke") {
            let hostname = matches.value_of("host").unwrap();
            subcommand_group::revoke(&mut db, &group_id, &hostname);
        } else if let Some(matches) = matches.subcommand_matches("user") {
            let user_id = matches.value_of("user").unwrap_or("");

            if matches.subcommand_matches("add").is_some() {
                subcommand_group::user_add(&mut db, &group_id, &user_id);
            } else if matches.subcommand_matches("remove").is_some() {
                subcommand_group::user_remove(&mut db, &group_id, &user_id);
            }
        }
    }
    // sync
    else if matches.subcommand_matches("sync").is_some() {
        subcommand_sync::sync(&mut db);
    }

    // save database
    db.save(&database_file);
}

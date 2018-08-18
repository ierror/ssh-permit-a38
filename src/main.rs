#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate clap;
extern crate colored;
extern crate difference;
extern crate rpassword;
extern crate serde;
extern crate serde_json;
extern crate ssh2;

use clap::{App, Arg, SubCommand};
use std::path::Path;

mod cli_flow;
mod database;
mod ssh_config;
mod subcommand_group;
mod subcommand_host;
mod subcommand_howto;
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
                .alias("hosts")

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
                    // --raw
                    .arg(
                        Arg::with_name("raw")
                            .short("r")
                            .long("raw")
                            .help("Prints raw host struct")
                    )
                )
                // host <host> alias <alias>
                .subcommand(
                    SubCommand::with_name("alias")
                        .arg(Arg::with_name("alias")
                            .help("Host alias")
                            .index(1)
                            .required(false))
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
                .alias("users")

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
                    // --raw
                    .arg(
                        Arg::with_name("raw")
                            .short("r")
                            .long("raw")
                            .help("Prints raw host struct"))
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
                .alias("groups")

                // group <group> add
                .subcommand(
                    SubCommand::with_name("add")
                        // group <group> add <user>
                        .arg(Arg::with_name("user")
                        .help("User")
                        .index(1)
                        .required(false))
                )
                // group <group> remove
                .subcommand(
                    SubCommand::with_name("remove")
                        // group <group> remove <user>
                        .arg(Arg::with_name("user")
                        .help("User")
                        .index(1)
                        .required(false))
                )
                // group list
                .subcommand(
                    SubCommand::with_name("list")
                    // --raw
                    .arg(
                        Arg::with_name("raw")
                            .short("r")
                            .long("raw")
                            .help("Prints raw host struct")
                    )
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
        )

        // sync
        .subcommand(
            SubCommand::with_name("sync")
                .about("Sync pending changes to the related hosts")
                // --password
                .arg(
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .help("Use password authentication instead of public key")
                        .takes_value(false),
                )
                // --yes-authorized-keys-prompt
                .arg(
                    Arg::with_name("yes_authorized_keys_prompt")
                        .short("yakp")
                        .long("yes-authorized-keys-prompt")
                        .help("Automatic yes to authorized_keys location prompts")
                        .takes_value(false),
                )
        )
        // howto
        .subcommand(
            SubCommand::with_name("howto")
                .about("Prints a Howto")
        )
        .get_matches();

    // load database
    let database_file = matches.value_of("database").unwrap_or("ssh-permit.json");

    let mut db = database::Database {
        ..Default::default()
    };

    if Path::new(database_file).exists() {
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
    } else {
        cli_flow::warningln(&format!(
            "Database file {} does not exist. Created a new one.",
            database_file,
        ));
    }

    // host
    if let Some(matches) = matches.subcommand_matches("host") {
        let hostname = matches.value_of("host:port").unwrap_or("");

        if matches.subcommand_matches("add").is_some() {
            subcommand_host::add(&mut db, &hostname);
        } else if matches.subcommand_matches("remove").is_some() {
            subcommand_host::remove(&mut db, &hostname);
        } else if let Some(matches) = matches.subcommand_matches("list") {
            subcommand_host::list(&mut db, &hostname, matches.is_present("raw"));
        } else if let Some(matches) = matches.subcommand_matches("alias") {
            subcommand_host::alias(&mut db, &hostname, matches.value_of("alias"));
        }
    }
    // user
    else if let Some(matches) = matches.subcommand_matches("user") {
        let user_id = matches.value_of("user").unwrap_or("");

        if matches.subcommand_matches("add").is_some() {
            subcommand_user::add(&mut db, &user_id);
        } else if matches.subcommand_matches("remove").is_some() {
            subcommand_user::remove(&mut db, &user_id);
        } else if let Some(matches) = matches.subcommand_matches("list") {
            subcommand_user::list(&mut db, &user_id, matches.is_present("raw"));
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

        if let Some(matches) = matches.subcommand_matches("add") {
            match matches.value_of("user") {
                Some(user_id) => subcommand_group::user_add(&mut db, &group_id, &user_id),
                None => subcommand_group::add(&mut db, &group_id),
            }
        } else if let Some(matches) = matches.subcommand_matches("remove") {
            match matches.value_of("user") {
                Some(user_id) => subcommand_group::user_remove(&mut db, &group_id, &user_id),
                None => subcommand_group::remove(&mut db, &group_id),
            }
        } else if let Some(matches) = matches.subcommand_matches("list") {
            subcommand_group::list(&mut db, &group_id, matches.is_present("raw"));
        } else if let Some(matches) = matches.subcommand_matches("grant") {
            let hostname = matches.value_of("host").unwrap();
            subcommand_group::grant(&mut db, &group_id, &hostname);
        } else if let Some(matches) = matches.subcommand_matches("revoke") {
            let hostname = matches.value_of("host").unwrap();
            subcommand_group::revoke(&mut db, &group_id, &hostname);
        }
    }
    // sync
    else if let Some(matches) = matches.subcommand_matches("sync") {
        subcommand_sync::sync(
            &mut db,
            matches.is_present("password"),
            matches.is_present("yes_authorized_keys_prompt"),
        );
    }
    // howto
    else if matches.subcommand_matches("howto").is_some() {
        subcommand_howto::print();
    }

    // save database
    db.save(&database_file);
}

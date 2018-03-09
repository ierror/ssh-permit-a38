extern crate chrono;
extern crate colored;
extern crate serde;
extern crate serde_json;
extern crate ssh2;

#[macro_use]
extern crate serde_derive;
extern crate clap;

use clap::{App, Arg, SubCommand};
use std::io;

mod database;
mod flow;

fn main() {
    let matches = App::new("SSH Permit A38")
        // application info
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
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
                        .arg(Arg::with_name("host")
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
        .get_matches();

    let database_file = matches.value_of("database").unwrap_or("ssh-permit.json");
    println!("Value for config: {}", database_file);

    let mut db = database::Database {
        ..Default::default()
    }.load(database_file)
        .unwrap();

    // host
    if let Some(matches) = matches.subcommand_matches("host") {
        let hostname = matches.value_of("host").unwrap_or("");

        if matches.subcommand_matches("add").is_some() {
            {
                if db.host_get(hostname).is_some() {
                    flow::error(&format!("Hostname {} already exists", hostname));
                }
            }

            {
                // add new host
                let mut host_new = vec![
                    database::Host {
                        hostname: hostname.to_owned(),
                        ..Default::default()
                    },
                ];
                db.hosts.append(&mut host_new);
            }
        } else if matches.subcommand_matches("remove").is_some() {
            if !db.host_get(hostname).is_some() {
                flow::error(&format!("Hostname {} not known", hostname));
            }
            db.hosts.retain(|h| h.hostname != hostname);
        } else if matches.subcommand_matches("list").is_some() {
            for host in &db.hosts {
                println!("\n{}", host.hostname);
                println!(
                    "{}",
                    (0..host.hostname.len()).map(|_| "=").collect::<String>()
                );

                println!("\n## Authorized Users");
                for user in &host.authorized_users {
                    println!("* {}", user);
                }

                println!("\n## Authorized Groups");
                for group in &host.authorized_user_groups {
                    println!("\n* {}", group);
                }
            }
        }
    }

    // user
    if let Some(matches) = matches.subcommand_matches("user") {
        let user_id = matches.value_of("user").unwrap_or("");

        if matches.subcommand_matches("add").is_some() {
            // check user is not present
            if db.user_get(user_id).is_some() {
                flow::error(&format!("User {} already exists", user_id));
            }

            // read public key
            flow::info(&format!(
                "Paste the public key of {} and press the Enter key:",
                user_id
            ));
            let mut public_key = String::new();
            io::stdin()
                .read_line(&mut public_key)
                .ok()
                .expect("Couldn't read public key");

            // TODO:; daring assumption, validate...
            if !public_key.starts_with("ssh-") {
                flow::error("Invalid public ssh key format")
            }

            // add new user
            let mut user_new = vec![
                database::User {
                    user_id: user_id.to_owned(),
                    public_key: public_key.trim_right().trim_left().to_owned(),
                },
            ];

            db.users.append(&mut user_new);
        } else if matches.subcommand_matches("remove").is_some() {
            // check user exist
            if db.user_get(user_id).is_none() {
                flow::error(&format!("User {} not known", user_id));
            }

            db.users.retain(|u| u.user_id != user_id);
        } else if let Some(_matches) = matches.subcommand_matches("list") {
            for user in &db.users {
                println!("\n{}", user.user_id);
                println!(
                    "{}",
                    (0..user.user_id.len()).map(|_| "=").collect::<String>()
                );
            }
        } else if let Some(matches) = matches.subcommand_matches("grant") {
            let hostname = matches.value_of("host").unwrap();

            if let Some(host) = db.host_get(hostname) {
                if let Some(user) = db.user_get(user_id) {
                    if db.is_user_granted(&user, &host) {
                        flow::error(&format!(
                            "{} already granted to access {}",
                            user.user_id, hostname
                        ));
                    }
                } else {
                    flow::error(&format!("User {} not known", user_id));
                }
            } else {
                flow::error(&format!("Hostname {} not known", hostname));
            }

            // at this point it's save to mut db.host...
            {
                let host = db.host_get_mut(hostname).unwrap();
                host.authorized_users
                    .append(&mut vec![String::from(user_id)]);
            }
        } else if let Some(matches) = matches.subcommand_matches("revoke") {
            let hostname = matches.value_of("host").unwrap();

            if let Some(host) = db.host_get(hostname) {
                if let Some(user) = db.user_get(user_id) {
                    if !db.is_user_granted(&user, &host) {
                        flow::error(&format!(
                            "{} is not granted to access {}",
                            user.user_id, hostname
                        ));
                    }
                } else {
                    flow::error(&format!("User {} not known", user_id));
                }
            } else {
                flow::error(&format!("Hostname {} not known", hostname));
            }

            // at this point it's save to mut db.host...
            {
                let host = db.host_get_mut(hostname).unwrap();
                host.authorized_users.retain(|u| u != user_id);
            }
        }
    }

    /*    let user = database::User {
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
    };*/

    db.save(&database_file);
}

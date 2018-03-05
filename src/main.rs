extern crate chrono;
extern crate colored;
extern crate serde;
extern crate serde_json;
extern crate ssh2;

#[macro_use]
extern crate serde_derive;
extern crate clap;

use clap::{App, Arg, SubCommand};

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
                // host add
                .about("Add or remove a host")
                .subcommand(
                    SubCommand::with_name("add")
                        .arg(Arg::with_name("host")
                            .help("Add a new host")
                            .index(1)
                            .required(true)),
                )
                // host remove
                .subcommand(
                    SubCommand::with_name("remove")
                        .arg(Arg::with_name("host")
                            .help("Remove a host")
                            .index(1)
                            .required(true)),
                )
                // host list
                .subcommand(
                    SubCommand::with_name("list")
                ),
        )
        // User
        .subcommand(
            SubCommand::with_name("user")
                // user add
                .about("Add or remove a user")
                .subcommand(
                    SubCommand::with_name("add")
                        .arg(Arg::with_name("user")
                            .help("Add a new user")
                            .index(1)
                            .required(true)),
                )
                // user remove
                .subcommand(
                    SubCommand::with_name("remove")
                        .arg(Arg::with_name("user")
                            .help("Remove an user")
                            .index(1)
                            .required(true)),
                )
                // user  list
                .subcommand(
                    SubCommand::with_name("list")
                ),
        )
        .get_matches();

    let database_file = matches.value_of("database").unwrap_or("ssh-permit.json");
    println!("Value for config: {}", database_file);

    let mut db = database::load(database_file).unwrap();

    // host
    if let Some(matches) = matches.subcommand_matches("host") {
        if let Some(matches) = matches.subcommand_matches("add") {
            let hostname_to_add = matches.value_of("host").unwrap();

            // check host is not present
            let index = db.hosts
                .iter()
                .position(|ref h| h.hostname == hostname_to_add);
            if !index.is_none() {
                flow::error(format!("Hostname {} already exists.", hostname_to_add));
            }

            // add new host
            let mut host_new = vec![
                database::Host {
                    hostname: hostname_to_add.to_owned(),
                    authorized_users: vec![],
                    authorized_groups: vec![],
                },
            ];

            db.hosts.append(&mut host_new);
        } else if let Some(matches) = matches.subcommand_matches("remove") {
            let hostname_to_del = matches.value_of("host").unwrap();

            // check host exist
            let index = db.hosts
                .iter()
                .position(|ref h| h.hostname == hostname_to_del);
            if index.is_none() {
                flow::error(format!("Hostname {} not known.", hostname_to_del));
            }

            db.hosts.retain(|h| h.hostname != hostname_to_del);
        } else if let Some(_matches) = matches.subcommand_matches("list") {
            for host in &db.hosts {
                println!("\n{}", host.hostname);
                println!("{}", (0..host.hostname.len()).map(|_| "=").collect::<String>());

                println!("\n## Authorized Users");
                for user in &host.authorized_users {
                    println!("\n* {}", user);
                }

                println!("\n## Authorized Groups");
                for group in &host.authorized_groups {
                    println!("\n* {}", group);
                }
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

    /*    use ssh2::Session;

    // Almost all APIs require a `Session` to be available
    let sess = Session::new().unwrap();
    let mut agent = sess.agent().unwrap();

    // Connect the agent and request a list of identities
    agent.connect().unwrap();
    agent.list_identities().unwrap();

    for identity in agent.identities() {
        //let identity = identity.unwrap(); // assume no I/O errors
        //let pubkey = identity.blob();
        //println!(">>>>>>>>>>>>>>{}", identity);
    }*/

    database::save(&database_file, &mut db);
}

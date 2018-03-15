use cli_flow;
use colored::Colorize;
use database::Database;
use difference::{Changeset, Difference};
use ssh2::Session;
use std::io::Read;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use std::str;

pub fn sync(db: &mut Database) {
    let mut syned_sth = false;

    for host in &mut db.hosts {
        // sync needed for host?
        if !host.sync_todo {
            continue;
        }

        cli_flow::infoln(&format!("Syncing host {}...", host.hostname));
        syned_sth = true;

        // ssh connect to host
        // defaults for connection
        let mut hostname = &*host.hostname;
        let mut port = "22";

        // hostname:port format?
        let host_splitted: Vec<&str> = host.hostname.split(':').collect();

        // found one ':' in hostname
        if host_splitted.len() == 2 {
            if host_splitted[1].parse::<i32>().is_ok() {
                hostname = &*host_splitted[0];
                port = &*host_splitted[1];
            }
        }

        // connect!
        let tcp = match TcpStream::connect(&format!("{}:{}", hostname, port)) {
            Ok(t) => t,
            Err(e) => {
                cli_flow::warningln(&format!("{}: {}", host.hostname, e.to_string()));
                continue;
            }
        };

        let mut sess = Session::new().unwrap();
        sess.handshake(&tcp).unwrap();

        let mut agent = sess.agent().unwrap();

        // Connect the agent and request a list of identities
        agent.connect().unwrap();
        agent.list_identities().unwrap();

        // try public key authentication
        sess.userauth_pubkey_file(
            "root",
            Some(Path::new("/Users/boerni/.ssh/id_rsa.pub")),
            Path::new("/Users/boerni/.ssh/id_rsa"),
            Some("TBD"),
        ).unwrap();

        // connection succesfull?
        //println!("{}", sess.authenticated());

        // read current authorized_keys from host
        let mut channel = sess.channel_session().unwrap();
        channel.exec("echo $HOME").unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        let remote_authorized_keys = format!(
            "{}/.ssh/authorized_keys",
            s.trim_right().trim_left().to_owned()
        );
        channel.wait_close().is_ok();
        //println!(
        //    "{} {}",
        //    channel.exit_status().unwrap(),
        //    remote_authorized_keys
        //);

        let (mut remote_file, stat) = sess.scp_recv(Path::new(&remote_authorized_keys)).unwrap();
        //println!("remote file size: {}", stat.size());
        let mut contents = Vec::new();
        remote_file.read_to_end(&mut contents).unwrap();

        let s = match str::from_utf8(&contents) {
            Ok(v) => v,
            Err(e) => {
                cli_flow::warningln(&format!("{}: Invalid UTF-8 sequence: {}", host.hostname, e));
                continue;
            }
        };

        // collect authorized_keys to sync ...
        let mut authorized_keys_vec: Vec<String> = Vec::new();

        // ... 1. on user level
        for authorized_user_id in &host.authorized_users {
            for user in &db.users {
                if &user.user_id == authorized_user_id {
                    // build e.g.
                    // # mail@example.com
                    // ssh-rsa ...
                    authorized_keys_vec.append(&mut vec![
                        format!(
                            "# {}\n{}",
                            authorized_user_id,
                            String::from(&*user.public_key)
                        ),
                    ]);
                }
            }
        }

        // ... 2. on group level
        for authorized_group_id in &host.authorized_user_groups {
            for group in &db.user_groups {
                if authorized_group_id == &group.group_id {
                    for user_id in &group.members {
                        for user in &db.users {
                            if user_id == &user.user_id {
                                authorized_keys_vec.append(&mut vec![
                                    format!("# {}\n{}", user_id, String::from(&*user.public_key)),
                                ]);
                                break;
                            }
                        }
                    }
                    break;
                }
            }
        }

        authorized_keys_vec.sort();
        authorized_keys_vec.dedup();

        // show diff of authorized_keys of host <-> to sync
        let authorized_keys_str = format!("{}\n", authorized_keys_vec.join("\n\n"));
        let Changeset { diffs, .. } = Changeset::new(s, &authorized_keys_str, "\n");

        println!("");
        for i in 0..diffs.len() {
            match diffs[i] {
                Difference::Same(ref x) => {
                    println!("{}", x);
                }
                Difference::Add(ref x) => {
                    println!("{}", format!("+{}", x).green());
                }
                Difference::Rem(ref x) => {
                    println!("{}", format!("-{}", x).red());
                }
            }
        }

        // sync confirmation
        if cli_flow::prompt_yes_no("Verify changes. Do you want to sync? (y/n):") == "n" {
            cli_flow::warningln(&format!("Skipping sync of {} as you told so\n\n", hostname));
            continue;
        }

        // sync!
        let remote_path = "/root/.ssh/authorized_keys";
        let mut remote_file = sess.scp_send(
            Path::new(remote_path),
            0o600,
            authorized_keys_str.len() as u64,
            None,
        ).unwrap();
        remote_file.write(authorized_keys_str.as_bytes()).unwrap();

        cli_flow::okln(&format!(
            "Successfully synced to {}:{}\n\n",
            hostname, remote_path
        ));

        host.sync_todo = false;
    }

    if syned_sth {
        cli_flow::okln("Successfully synced all hosts");
    } else {
        cli_flow::warningln("All hosts up to date. Nothing to sync, bye bye");
    }
}

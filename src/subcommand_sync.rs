use cli_flow;
use colored::Colorize;
use database::Database;
use difference::{Changeset, Difference};
use ssh2::Session;
use std::io;
use std::io::Read;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use std::str;

pub fn sync(db: &mut Database) {
    for mut host in &mut db.hosts {
        println!("");

        cli_flow::info(&format!("Syncing host {}", host.hostname));
        cli_flow::info(&format!(
            "{}",
            (0..host.hostname.len() + 13)
                .map(|_| "=")
                .collect::<String>()
        ));

        if !host.sync_todo {
            cli_flow::warning(&format!("{} is up to date\n\n", host.hostname));
            continue;
        }

        // defaults
        let mut hostname = &*host.hostname;
        let mut port = "22";

        let host_splitted: Vec<&str> = host.hostname.split(':').collect();

        // found one ':' in hostname
        if host_splitted.len() == 2 {
            if host_splitted[1].parse::<i32>().is_ok() {
                hostname = &*host_splitted[0];
                port = &*host_splitted[1];
            }
        }

        let tcp = match TcpStream::connect(&format!("{}:{}", hostname, port)) {
            Ok(t) => t,
            Err(e) => {
                cli_flow::warning(&format!("{}: {}", host.hostname, e.to_string()));
                continue;
            }
        };

        let mut sess = Session::new().unwrap();
        sess.handshake(&tcp).unwrap();

        let mut agent = sess.agent().unwrap();

        // Connect the agent and request a list of identities
        agent.connect().unwrap();
        agent.list_identities().unwrap();

        sess.userauth_pubkey_file(
            "root",
            Some(Path::new("/Users/boerni/.ssh/id_rsa.pub")),
            Path::new("/Users/boerni/.ssh/id_rsa"),
            Some("TBD"),
        ).unwrap();
        //println!("{}", sess.authenticated());

        let mut channel = sess.channel_session().unwrap();
        channel.exec("echo $HOME").unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        let remote_authorized_keys = format!(
            "{}/.ssh/authorized_keys",
            s.trim_right().trim_left().to_owned()
        );
        channel.wait_close();
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
                cli_flow::warning(&format!("{}: Invalid UTF-8 sequence: {}", host.hostname, e));
                continue;
            }
        };

        // collect authorized_keys for host
        let mut authorized_keys_vec: Vec<String> = Vec::new();
        for authorized_user_id in &mut host.authorized_users {
            for user in &mut db.users {
                if &user.user_id == authorized_user_id {
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

        authorized_keys_vec.sort();
        authorized_keys_vec.dedup();

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
        cli_flow::prompt("\nVerify changes. Do you want to sync (y/n)?");
        let mut deploy_yes_no = String::new();
        loop {
            io::stdin()
                .read_line(&mut deploy_yes_no)
                .ok()
                .expect("Couldn't read (y/n) sync confirmation");

            deploy_yes_no = deploy_yes_no.trim_right().trim_left().to_owned();
            if deploy_yes_no == "n" || deploy_yes_no == "y" {
                break;
            }
        }

        if deploy_yes_no == "n" {
            cli_flow::warning(&format!("Skipping sync of {} as you told so\n\n", hostname));
            continue;
        }

        let mut remote_file = sess.scp_send(
            Path::new("/root/.ssh/authorized_keys"),
            0o600,
            authorized_keys_str.len() as u64,
            None,
        ).unwrap();
        remote_file.write(authorized_keys_str.as_bytes()).unwrap();

        //host.sync_todo = true;
        cli_flow::ok(&format!("Successfully synced to {}\n\n", hostname));
    }
}

use cli_flow;
use colored::Colorize;
use database::Database;
use difference::{Changeset, Difference};
use rpassword;
use ssh2::Session;
use ssh_config;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;
use std::str;

fn userauth_agent(sess: &mut Session, ssh_user: &str) -> Result<bool, Box<Error>> {
    let mut agent = try!(sess.agent());
    try!(agent.connect());
    agent.list_identities().unwrap();

    for identity in agent.identities() {
        let identity = try!(identity);
        if agent.userauth(&ssh_user, &identity).is_ok() {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn sync(db: &mut Database, password_auth: bool, yes_authorized_keys_prompt: bool) {
    let ssh_config = match ssh_config::get() {
        Ok(c) => c,
        Err(e) => {
            cli_flow::warningln(&e.to_string());
            HashMap::new()
        }
    };

    let mut syned_sth = false;

    for host in &mut db.hosts {
        // sync needed for host?
        if !host.sync_todo {
            continue;
        }

        println!("");
        cli_flow::infoln(&format!("# Syncing host {}...", host.hostname));
        println!("");

        syned_sth = true;

        // ssh connect to host
        // defaults for connection
        let mut ssh_host = &*host.hostname;
        let mut ssh_port = "22";

        let mut ssh_user = String::new();
        let ssh_user_default = "root";

        let mut ssh_config_used = false;

        // is host to connect found in ssh_config?
        for (cfg_host_label, cfg_host) in &ssh_config {
            if cfg_host_label == ssh_host
                || cfg_host_label == &(host.alias.clone()).unwrap_or("".to_string())
                || cfg_host.hostname == ssh_host
            {
                cli_flow::infoln(&format!(
                        "Found hostname or alias {} in ssh_config, using config parameters (hostname, user, port) for connection",
                        ssh_host
                    ));

                ssh_host = &cfg_host.hostname;
                ssh_user = cfg_host.user.to_owned();
                ssh_port = &cfg_host.port;
                ssh_config_used = true;

                break;
            }
        }

        // hostname:port format?
        if !ssh_config_used {
            let host_splitted: Vec<&str> = host.hostname.split(':').collect();

            // found one ':' in hostname
            if host_splitted.len() == 2 {
                if host_splitted[1].parse::<i32>().is_ok() {
                    ssh_host = &*host_splitted[0];
                    ssh_port = &*host_splitted[1];
                }
            }
        }

        // connect!
        let ssh_tcp = match TcpStream::connect(&format!("{}:{}", ssh_host, ssh_port)) {
            Ok(t) => t,
            Err(e) => {
                cli_flow::errorln(&e.to_string());
                continue;
            }
        };

        // create ssh session
        let mut ssh_sess = match Session::new() {
            Some(s) => s,
            None => {
                cli_flow::errorln("Unable to create SSH session.");
                continue;
            }
        };

        // ssh handshake
        match ssh_sess.handshake(&ssh_tcp) {
            Ok(h) => h,
            Err(e) => {
                cli_flow::errorln(&e.to_string());
                continue;
            }
        };

        // prompt for remote user
        if !ssh_config_used {
            ssh_user = cli_flow::read_line(
                &format!("SSH User ({}):", ssh_user_default),
                &ssh_user_default.to_owned(),
            ).to_owned();
        } else {
            cli_flow::infoln(&format!("SSH User: {}", ssh_user));
        }

        if password_auth {
            // prompt for password
            cli_flow::prompt("Password:", false);
            let password = rpassword::prompt_password_stdout("").unwrap();

            match ssh_sess.userauth_password(&ssh_user, &password) {
                Ok(t) => {
                    // drop ssh_password
                    drop(password);
                    t
                }
                Err(e) => {
                    cli_flow::errorln(&e.to_string());
                    // drop passphrase
                    drop(password);
                    continue;
                }
            };
        } else {
            let agent_authed = match userauth_agent(&mut ssh_sess, &ssh_user) {
                Ok(true) => true,
                Ok(false) | Err(_) => false,
            };

            if !agent_authed {
                // guess ssh key location
                let private_key_path = match env::home_dir() {
                    Some(path) => path.join(".ssh").join("id_rsa"),
                    None => Path::new("").to_path_buf(),
                };

                let mut private_key_file_default = private_key_path.to_str().unwrap();
                let private_key_file = &cli_flow::read_line(
                    &format!("Private key ({}):", private_key_file_default),
                    &private_key_file_default.to_owned(),
                );

                // prompt for passphrase
                cli_flow::prompt("Passphrase (empty for no passphrase):", false);
                let private_key_pass = rpassword::prompt_password_stdout("").unwrap();

                // public key auth
                match ssh_sess.userauth_pubkey_file(
                    &ssh_user,
                    None,
                    Path::new(&private_key_file),
                    Some(&private_key_pass),
                ) {
                    Ok(t) => {
                        // drop passphrase
                        drop(private_key_pass);
                        t
                    }
                    Err(e) => {
                        cli_flow::errorln(&e.to_string());
                        // drop passphrase
                        drop(private_key_pass);
                        continue;
                    }
                }
            }
        }

        // read current authorized_keys from host
        let mut remote_authorized_keys_file_default = String::new();

        if let Ok(mut channel) = ssh_sess.channel_session() {
            let mut r_get_home = channel.exec("echo $HOME");

            if r_get_home.is_ok() {
                let mut home = String::new();
                let r_read = channel.read_to_string(&mut home);

                if r_read.is_ok() {
                    remote_authorized_keys_file_default = format!(
                        "{}/.ssh/authorized_keys",
                        home.trim_right().trim_left().to_owned()
                    );
                    channel.wait_close().is_ok();
                }
            }
        };

        // prompt for remote authorized_keys file
        let mut remote_authorized_keys_file = String::new();

        if yes_authorized_keys_prompt {
            cli_flow::infoln(&format!(
                "Remote authorized_keys: {}",
                remote_authorized_keys_file_default
            ));
        } else {
            remote_authorized_keys_file = cli_flow::read_line(
                &format!(
                    "Remote authorized_keys ({}):",
                    remote_authorized_keys_file_default
                ),
                &remote_authorized_keys_file_default,
            ).to_owned();
        }

        let authorized_keys_res = ssh_sess.scp_recv(Path::new(&remote_authorized_keys_file));
        let mut authorized_keys_remote = Vec::new();
        let mut authorized_keys_remote_str = "";

        match authorized_keys_res {
            Ok(r) => {
                let (mut ch, _stat) = r;
                ch.read_to_end(&mut authorized_keys_remote).unwrap();

                authorized_keys_remote_str = match str::from_utf8(&authorized_keys_remote) {
                    Ok(v) => v,
                    Err(e) => {
                        cli_flow::warningln(&format!(
                            "{}: Invalid UTF-8 sequence: {}",
                            host.hostname, e
                        ));
                        continue;
                    }
                };
            }
            Err(e) => {
                cli_flow::warningln(&format!(
                    "Unable to read remote {} - {}",
                    remote_authorized_keys_file,
                    e.to_string()
                ));
            }
        };

        // collect authorized_keys to sync ...
        let mut authorized_keys_sync_vec: Vec<String> = Vec::new();

        // ... 1. on user level
        for authorized_user_id in &host.authorized_users {
            for user in &db.users {
                if &user.user_id == authorized_user_id {
                    // build e.g.
                    // # mail@example.com
                    // ssh-rsa ...
                    authorized_keys_sync_vec.append(&mut vec![format!(
                        "# {}\n{}",
                        authorized_user_id,
                        String::from(&*user.public_key)
                    )]);
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
                                authorized_keys_sync_vec.append(&mut vec![format!(
                                    "# {}\n{}",
                                    user_id,
                                    String::from(&*user.public_key)
                                )]);
                                break;
                            }
                        }
                    }
                    break;
                }
            }
        }

        authorized_keys_sync_vec.sort();
        authorized_keys_sync_vec.dedup();

        // show diff of authorized_keys of host <-> to sync
        let authorized_keys_sync_str = format!("{}\n", authorized_keys_sync_vec.join("\n\n"));
        let Changeset { diffs, .. } =
            Changeset::new(&authorized_keys_remote_str, &authorized_keys_sync_str, "\n");

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
        if cli_flow::prompt_yes_no(
            &mut format!(
                "Verify changes. Do you want to sync to {}? (y/n):",
                remote_authorized_keys_file
            ),
            true,
        ) == "n"
        {
            cli_flow::warningln(&format!("Skipping sync of {} as you told so\n\n", ssh_host));
            continue;
        }

        // sync!
        let mut remote_authorized_keys_fh = match ssh_sess.scp_send(
            Path::new(&remote_authorized_keys_file),
            0o600,
            authorized_keys_sync_str.len() as u64,
            None,
        ) {
            Ok(r) => r,
            Err(e) => {
                cli_flow::errorln(&format!(
                    "Unable to upload {} - {}",
                    remote_authorized_keys_file,
                    &e.to_string()
                ));
                return;
            }
        };

        match remote_authorized_keys_fh.write(authorized_keys_sync_str.as_bytes()) {
            Ok(r) => r,
            Err(e) => {
                cli_flow::errorln(&format!(
                    "Unable to upload {} - {}",
                    remote_authorized_keys_file,
                    &e.to_string()
                ));
                return;
            }
        };

        // mark as synced
        host.sync_todo = false;

        cli_flow::okln(&format!(
            "Successfully synced to {}\n",
            remote_authorized_keys_file
        ));
    }

    if !syned_sth {
        cli_flow::okln("All hosts up to date. Nothing to sync, bye bye");
    }
}

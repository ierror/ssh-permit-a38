use cli_flow;
use database::{Database, User};
use std::io;

pub fn add(db: &mut Database, user_id: &str, pkey: &str) {
    // check user is not present
    if db.user_get(user_id).is_some() {
        cli_flow::errorln(&format!("User {} already exists", user_id));
    }

    let mut public_key = String::new();

    if pkey.len() > 0 {
        public_key = pkey.to_string();
    } else {
        // read public key
        cli_flow::promptln(&format!(
            "Paste the public key of {} and press the Enter key:",
            user_id
        ));

        io::stdin()
            .read_line(&mut public_key)
            .ok()
            .expect("Couldn't read public key");
    }
    // TODO:; daring assumption, validate...
    if !public_key.starts_with("ssh-") {
        cli_flow::errorln("Invalid public ssh key format")
    }

    // add new user
    let mut user_new = vec![User {
        user_id: user_id.to_owned(),
        public_key: public_key.trim_right().trim_left().to_owned(),
    }];

    db.users.append(&mut user_new);
    cli_flow::okln(&format!("Successfully added user {}", user_id));
}

pub fn remove(db: &mut Database, user_id: &str) {
    // check user exist
    if db.user_get(user_id).is_none() {
        cli_flow::errorln(&format!("User {} not known", user_id));
    }

    // rm user
    db.users.retain(|u| u.user_id != user_id);

    // delete user from hosts.authorized_users
    for host in &mut db.hosts {
        if (host.authorized_users.contains(&user_id.to_string())) {
            host.sync_todo = true;
        }
        host.authorized_users.retain(move |u| u != user_id);
    }

    // delete user from user_groups.members
    for user_group in &mut db.user_groups {
        user_group.members.retain(move |u| u != user_id);
    }

    cli_flow::okln(&format!("Successfully removed user {}", user_id));
}

pub fn list(db: &mut Database, user_id_filter: &str, print_raw: bool) {
    for user in &db.users {
        if !user_id_filter.is_empty() && user_id_filter != user.user_id {
            continue;
        }

        if print_raw {
            println!("{:?}", user);
            continue;
        }

        println!("\n{}", user.user_id);
        println!(
            "{}",
            (0..user.user_id.len()).map(|_| "=").collect::<String>()
        );
    }

    println!("");
}

pub fn grant(db: &mut Database, user_id: &str, hostname: &str) {
    if let Some(host) = db.host_get(hostname) {
        if let Some(user) = db.user_get(user_id) {
            if db.is_user_granted(&user, &host) {
                cli_flow::errorln(&format!(
                    "{} already granted to access {}",
                    user.user_id, hostname
                ));
            }
        } else {
            cli_flow::errorln(&format!("User {} not known", user_id));
        }
    } else {
        cli_flow::errorln(&format!("Hostname {} not known", hostname));
    }

    // at this point it's save to mut db.host...
    {
        let host = db.host_get_mut(hostname).unwrap();
        host.authorized_users
            .append(&mut vec![String::from(user_id)]);
        host.sync_todo = true;
    }

    cli_flow::okln(&format!(
        "Successfully granted user {} to host {}",
        user_id, hostname
    ));
}

pub fn revoke(db: &mut Database, user_id: &str, hostname: &str) {
    if let Some(host) = db.host_get(hostname) {
        if let Some(user) = db.user_get(user_id) {
            if !db.is_user_granted(&user, &host) {
                cli_flow::errorln(&format!(
                    "{} is not granted to access {}",
                    user.user_id, hostname
                ));
            }
        } else {
            cli_flow::errorln(&format!("User {} not known", user_id));
        }
    } else {
        cli_flow::errorln(&format!("Hostname {} not known", hostname));
    }

    // at this point it's save to mut db.host...
    {
        let host = db.host_get_mut(hostname).unwrap();
        host.authorized_users.retain(|u| u != user_id);
        host.sync_todo = true;
    }

    cli_flow::okln(&format!(
        "Successfully revoked user {} from host {}",
        user_id, hostname
    ));
}

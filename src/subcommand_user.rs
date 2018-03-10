use cli_flow;
use database::{Database, User};
use std::io;

pub fn add(db: &mut Database, user_id: &str) {
    // check user is not present
    if db.user_get(user_id).is_some() {
        cli_flow::error(&format!("User {} already exists", user_id));
    }

    // read public key
    cli_flow::prompt(&format!(
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
        cli_flow::error("Invalid public ssh key format")
    }

    // add new user
    let mut user_new = vec![
        User {
            user_id: user_id.to_owned(),
            public_key: public_key.trim_right().trim_left().to_owned(),
        },
    ];

    db.users.append(&mut user_new);
}

pub fn remove(db: &mut Database, user_id: &str) {
    // check user exist
    if db.user_get(user_id).is_none() {
        cli_flow::error(&format!("User {} not known", user_id));
    }

    db.users.retain(|u| u.user_id != user_id);
}

pub fn list(db: &mut Database) {
    for user in &db.users {
        println!("\n{}", user.user_id);
        println!(
            "{}",
            (0..user.user_id.len()).map(|_| "=").collect::<String>()
        );
    }
}

pub fn grant(db: &mut Database, user_id: &str, hostname: &str) {
    if let Some(host) = db.host_get(hostname) {
        if let Some(user) = db.user_get(user_id) {
            if db.is_user_granted(&user, &host) {
                cli_flow::error(&format!(
                    "{} already granted to access {}",
                    user.user_id, hostname
                ));
            }
        } else {
            cli_flow::error(&format!("User {} not known", user_id));
        }
    } else {
        cli_flow::error(&format!("Hostname {} not known", hostname));
    }

    // at this point it's save to mut db.host...
    {
        let host = db.host_get_mut(hostname).unwrap();
        host.authorized_users
            .append(&mut vec![String::from(user_id)]);
        host.sync_todo = true;
    }
}

pub fn revoke(db: &mut Database, user_id: &str, hostname: &str) {
    if let Some(host) = db.host_get(hostname) {
        if let Some(user) = db.user_get(user_id) {
            if !db.is_user_granted(&user, &host) {
                cli_flow::error(&format!(
                    "{} is not granted to access {}",
                    user.user_id, hostname
                ));
            }
        } else {
            cli_flow::error(&format!("User {} not known", user_id));
        }
    } else {
        cli_flow::error(&format!("Hostname {} not known", hostname));
    }

    // at this point it's save to mut db.host...
    {
        let host = db.host_get_mut(hostname).unwrap();
        host.authorized_users.retain(|u| u != user_id);
    }
}

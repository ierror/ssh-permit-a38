use cli_flow;
use database::{Database, UserGroup};

pub fn add(db: &mut Database, group_id: &str) {
    // check group is not present
    if db.group_get(group_id).is_some() {
        cli_flow::errorln(&format!("Group {} already exists", group_id));
    }

    // add new group
    let mut group_new = vec![
        UserGroup {
            group_id: group_id.to_owned(),
            members: vec![],
        },
    ];

    db.user_groups.append(&mut group_new);
    cli_flow::okln(&format!("Successfully added group {}", group_id));
}

pub fn remove(db: &mut Database, group_id: &str) {
    // check group exist
    if db.group_get(group_id).is_none() {
        cli_flow::errorln(&format!("Group {} not known", group_id));
    }

    // delete grouo
    db.user_groups.retain(|u| u.group_id != group_id);

    // delete user from user_groups.members
    for host in &mut db.hosts {
        host.authorized_user_groups.retain(move |g| g != group_id);
    }

    cli_flow::okln(&format!("Successfully removed group {}", group_id));
}

pub fn list(db: &mut Database, group_filter: &str, print_raw: bool) {
    for group in &db.user_groups {
        if !group_filter.is_empty() && group_filter != group.group_id {
            continue;
        }

        if print_raw {
            println!("{:?}", group);
            continue;
        }

        println!("\n{}", group.group_id);
        println!(
            "{}",
            (0..group.group_id.len()).map(|_| "=").collect::<String>()
        );

        println!("\n## Members");
        for user in &group.members {
            println!("* {}", user);
        }
    }

    println!("");
}

pub fn grant(db: &mut Database, group_id: &str, hostname: &str) {
    if let Some(host) = db.host_get(hostname) {
        if let Some(group) = db.group_get(group_id) {
            if db.is_group_granted(&group, &host) {
                cli_flow::errorln(&format!(
                    "{} already granted to access {}",
                    group.group_id, hostname
                ));
            }
        } else {
            cli_flow::errorln(&format!("Group {} not known", group_id));
        }
    } else {
        cli_flow::errorln(&format!("Hostname {} not known", hostname));
    }

    // at this point it's save to mut db.host...
    {
        let host = db.host_get_mut(hostname).unwrap();
        host.authorized_user_groups
            .append(&mut vec![String::from(group_id)]);
        host.sync_todo = true;
    }

    cli_flow::okln(&format!(
        "Successfully granted group {} for host {}",
        group_id, hostname
    ));
}

pub fn revoke(db: &mut Database, group_id: &str, hostname: &str) {
    if let Some(host) = db.host_get(hostname) {
        if let Some(group) = db.group_get(group_id) {
            if !db.is_group_granted(&group, &host) {
                cli_flow::errorln(&format!(
                    "{} is not granted to access {}",
                    group.group_id, hostname
                ));
            }
        } else {
            cli_flow::errorln(&format!("Group {} not known", group_id));
        }
    } else {
        cli_flow::errorln(&format!("Hostname {} not known", hostname));
    }

    // at this point it's save to mut db.host...
    {
        let host = db.host_get_mut(hostname).unwrap();
        host.authorized_user_groups.retain(|u| u != group_id);
        host.sync_todo = true;
    }

    cli_flow::okln(&format!(
        "Successfully revoked group {} from host {}",
        group_id, hostname
    ));
}

pub fn user_add(db: &mut Database, group_id: &str, user_id: &str) {
    // check user and group exist
    if let Some(user) = db.user_get(user_id) {
        if let Some(group) = db.group_get(group_id) {
            if db.is_user_group_member(&user, &group) {
                cli_flow::errorln(&format!(
                    "User {} is already member of group {}",
                    user_id, group_id
                ));
            }
        } else {
            cli_flow::errorln(&format!("Group {} not known", group_id));
        }
    } else {
        cli_flow::errorln(&format!("User {} not known", user_id));
    }

    // at this point it's save to mut db.host...
    {
        let group = db.group_get_mut(group_id).unwrap();
        group.members.append(&mut vec![String::from(user_id)]);
    }
    {
        // set sync todo for affected hosts
        for host in &mut db.hosts {
            for authorized_group in &mut host.authorized_user_groups {
                if authorized_group == group_id {
                    host.sync_todo = true;
                }
            }
        }
    }

    cli_flow::okln(&format!(
        "Successfully added user {} to group {}",
        user_id, group_id
    ));
}

pub fn user_remove(db: &mut Database, group_id: &str, user_id: &str) {
    // check user and group exist
    if let Some(user) = db.user_get(user_id) {
        if let Some(group) = db.group_get(group_id) {
            if !db.is_user_group_member(&user, &group) {
                cli_flow::errorln(&format!(
                    "User {} is not a member of group {}",
                    user_id, group_id
                ));
            }
        } else {
            cli_flow::errorln(&format!("Group {} not known", group_id));
        }
    } else {
        cli_flow::errorln(&format!("User {} not known", user_id));
    }

    // at this point it's save to mut db.host...
    {
        let group = db.group_get_mut(group_id).unwrap();
        group.members.retain(|u| u != user_id);
    }
    {
        // set sync todo for affected hosts
        for host in &mut db.hosts {
            for authorized_group in &mut host.authorized_user_groups {
                if authorized_group == group_id {
                    host.sync_todo = true;
                }
            }
        }
    }

    cli_flow::okln(&format!(
        "Successfully removed user {} from group {}",
        group_id, group_id
    ));
}

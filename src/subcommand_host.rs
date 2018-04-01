use cli_flow;
use database::{Database, Host};

pub fn add(db: &mut Database, hostname: &str) {
    if db.host_get(hostname).is_some() {
        cli_flow::errorln(&format!(
            "Hostname or a host alias {} already exists",
            hostname
        ));
    }

    // <= 1 char ':' allowed
    if hostname.matches(":").count() > 1 {
        cli_flow::errorln("Hostname format invalid. More than than one ':' found");
    }

    // check that port part is integer
    let host_splitted: Vec<&str> = hostname.split(':').collect();
    if host_splitted.len() == 2 {
        if !host_splitted[1].parse::<i32>().is_ok() {
            cli_flow::errorln("Hostname format invalid. Port is not a integer");
        }
    }

    // add new host
    let mut host_new = vec![
        Host {
            hostname: hostname.to_owned(),
            ..Default::default()
        },
    ];

    db.hosts.append(&mut host_new);
    cli_flow::okln(&format!("Successfully added host {}", hostname));
}

pub fn remove(db: &mut Database, hostname: &str) {
    if !db.host_get(hostname).is_some() {
        cli_flow::errorln(&format!("Hostname {} not known", hostname));
    }

    db.hosts.retain(|h| h.hostname != hostname);
    cli_flow::okln(&format!("Successfully removed host {}", hostname));
}

pub fn list(db: &mut Database, hostname_filter: &str, print_raw: bool) {
    for host in &db.hosts {
        if !hostname_filter.is_empty()
            && (hostname_filter != host.hostname && Some(hostname_filter.to_owned()) != host.alias)
        {
            continue;
        }

        if print_raw {
            println!("{:?}", host);
            continue;
        }

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
            println!("* {}", group);
        }

        println!("");
    }
}

pub fn alias(db: &mut Database, hostname: &str, alias_opt: Option<&str>) {
    {
        // filter host by hostname only
        let host_lookup_by_hostname = db.hosts
            .iter()
            .position(|ref h| h.hostname == hostname)
            .map(|i| &db.hosts[i]);

        match host_lookup_by_hostname {
            Some(_h) => (),
            None => {
                cli_flow::errorln(&format!("Hostname {} does not exist", hostname));
                return;
            }
        }
    }
    {
        if alias_opt.is_some() {
            let alias = alias_opt.unwrap();

            if db.host_get(alias).is_some() {
                cli_flow::errorln(&format!("There is already a host with hostname or alias {} - You can't use an alias where a host with this hostname already exists.", alias));
            }

            if db.host_get_by_alias(alias).is_some() {
                cli_flow::errorln(&format!("Host alias {} already exists", alias));
            }
            {
                let host = db.host_get_mut(hostname).unwrap();
                host.alias = Some(alias.to_owned());

                cli_flow::okln(&format!(
                    "Successfully set alias {} for host {}",
                    alias, hostname
                ));
            }
        } else {
            let host = db.host_get_mut(hostname).unwrap();
            if !host.alias.is_some() {
                cli_flow::errorln(&format!("No alias set for host {}", hostname));
            }

            host.alias = None;
            cli_flow::okln(&format!("Successfully removed alias for host {}", hostname));
        }
    }
}

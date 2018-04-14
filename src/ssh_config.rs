use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub struct SSHConfigHost {
    pub hostname: String,
    pub user: String,
    pub port: String,
}

impl Default for SSHConfigHost {
    fn default() -> SSHConfigHost {
        SSHConfigHost {
            hostname: String::new(),
            user: String::new(),
            port: "22".to_owned(),
        }
    }
}

pub fn get() -> Result<HashMap<String, SSHConfigHost>, Box<Error>> {
    let mut ssh_config = HashMap::new();

    // guess ~/.ssh/config path
    let ssh_config_path = match env::home_dir() {
        Some(path) => path.join(".ssh").join("config"),
        None => Path::new("").to_path_buf(),
    };

    if !ssh_config_path.exists() {
        return Ok(ssh_config);
    }

    // open ssh config
    let ssh_config_file = match File::open(&ssh_config_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(From::from(format!(
                "SSH config file {} exists but can't be read - {}",
                ssh_config_path.to_str().unwrap(),
                &e.to_string()
            )));
        }
    };

    // parse file
    let ssh_config_reader = BufReader::new(&ssh_config_file);
    let mut host = String::new();
    for (_, line) in ssh_config_reader.lines().enumerate() {
        let mut line = line.unwrap();

        // Host
        if line.starts_with("Host") {
            host = line.split(" ").collect::<Vec<&str>>()[1].to_owned();
            ssh_config.insert(
                host.to_owned(),
                SSHConfigHost {
                    ..Default::default()
                },
            );
        }
        // config option
        else if !host.is_empty() && !line.trim().is_empty()
            && [' ', '\t'].contains(&line.chars().next().unwrap())
        {
            line = line.trim().to_owned();
            let option_value = line.split(" ").collect::<Vec<&str>>();
            let option = option_value[0].trim().to_lowercase();
            let value = option_value[1].trim().to_owned();
            let host_entry = ssh_config.get_mut(&host).unwrap();

            match option.as_str() {
                "hostname" => host_entry.hostname = value,
                "user" => host_entry.user = value,
                "port" => {
                    // port valid?
                    host_entry.port = match value.parse::<i32>() {
                        Ok(i) => i.to_string(),
                        Err(_e) => {
                            host_entry.port.to_owned() // keep default
                        }
                    }
                }
                _ => {}
            }
        } else {
            host = "".to_owned();
        }
    }

    Ok(ssh_config)
}

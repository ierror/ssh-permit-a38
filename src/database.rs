extern crate serde;
extern crate serde_json;

use chrono::Utc;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::Path;

const SCHEMA_VERSION: &'static str = "1.0.0";

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub hosts: Vec<Host>,
    pub users: Vec<User>,
    pub user_groups: Vec<UserGroup>,

    pub modified_at: String,
    pub schema_version: String,
}

impl Default for Database {
    fn default() -> Database {
        Database {
            hosts: vec![],
            users: vec![],
            user_groups: vec![],
            modified_at: String::from(""),
            schema_version: SCHEMA_VERSION.to_owned(),
        }
    }
}

impl Database {
    pub fn load<P: AsRef<Path>>(&self, path: P) -> Result<Database, Box<Error>> {
        let file = File::open(path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn save<P: AsRef<Path>>(&mut self, path: P) {
        let file = File::create(path).unwrap();
        let now = Utc::now();
        self.modified_at = format!("{}", now.to_owned());
        serde_json::to_writer_pretty(&file, &self).expect("Unable to write database file.");
    }

    pub fn host_get(&self, hostname: &str) -> Option<&Host> {
        self.hosts
            .iter()
            .position(|ref h| h.hostname == hostname)
            .map(|i| &self.hosts[i])
    }

    pub fn host_get_mut(&mut self, hostname: &str) -> Option<&mut Host> {
        self.hosts
            .iter()
            .position(|ref h| h.hostname == hostname)
            .map(move |i| &mut self.hosts[i])
    }

    pub fn user_get(&self, user_id: &str) -> Option<&User> {
        self.users
            .iter()
            .position(|u| u.user_id == user_id)
            .map(|i| &self.users[i])
    }

    pub fn user_get_mut(&mut self, user_id: &str) -> Option<&mut User> {
        self.users
            .iter()
            .position(|u| u.user_id == user_id)
            .map(move |i| &mut self.users[i])
    }

    pub fn group_get(&self, group_id: &str) -> Option<&UserGroup> {
        self.user_groups
            .iter()
            .position(|g| g.group_id == group_id)
            .map(|i| &self.user_groups[i])
    }

    pub fn group_get_mut(&mut self, group_id: &str) -> Option<&mut UserGroup> {
        self.user_groups
            .iter()
            .position(|g| g.group_id == group_id)
            .map(move |i| &mut self.user_groups[i])
    }

    pub fn is_user_granted(&self, user: &User, host: &Host) -> bool {
        host.authorized_users
            .iter()
            .position(|au| au == &user.user_id)
            .is_some()
    }

    pub fn is_group_granted(&self, user_group: &UserGroup, host: &Host) -> bool {
        host.authorized_user_groups
            .iter()
            .position(|ag| ag == &user_group.group_id)
            .is_some()
    }

    pub fn is_user_group_member(&self, user: &User, user_group: &UserGroup) -> bool {
        user_group
            .members
            .iter()
            .position(|u| u == &user.user_id)
            .is_some()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Host {
    pub hostname: String,
    pub authorized_users: Vec<String>,
    pub authorized_user_groups: Vec<String>,
    pub sync_todo: bool,
}

impl Default for Host {
    fn default() -> Host {
        Host {
            hostname: String::from(""),
            authorized_users: vec![],
            authorized_user_groups: vec![],
            sync_todo: true,
        }
    }
}

impl fmt::Display for Host {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hostname)
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub public_key: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.user_id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserGroup {
    pub group_id: String,
    pub members: Vec<String>,
}

impl fmt::Display for UserGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.group_id)
    }
}

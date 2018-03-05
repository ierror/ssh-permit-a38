extern crate serde;
extern crate serde_json;

use chrono::Utc;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::Path;

//const SCHEMA_VERSION: String = "1.0".to_string();

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub hosts: Vec<Host>,
    pub users: Vec<User>,
    pub user_groups: Vec<UserGroup>,

    pub modified_at: String,
    pub version: String,
}

impl Default for Database {
    fn default() -> Database {
        Database {
            hosts: vec![],
            users: vec![],
            user_groups: vec![],
            modified_at: "".to_owned(),
            version: "1.0".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Host {
    pub hostname: String,
    pub authorized_users: Vec<User>,
    pub authorized_user_groups: Vec<UserGroup>,
}

impl Default for Host {
    fn default() -> Host {
        Host {
            hostname: "".to_owned(),
            authorized_users: vec![],
            authorized_user_groups: vec![],
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
    pub name: String,
    pub public_key: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserGroup {
    pub name: String,
    pub members: Vec<User>,
}

impl fmt::Display for UserGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn load<P: AsRef<Path>>(path: P) -> Result<Database, Box<Error>> {
    let file = File::open(path)?;
    Ok(serde_json::from_reader(file)?)
}

pub fn save<P: AsRef<Path>>(path: P, db: &mut Database) -> () {
    let file = File::create(path).unwrap();
    let now = Utc::now();
    db.modified_at = format!("{}", now.to_owned());
    serde_json::to_writer_pretty(&file, &db).expect("Unable to write database file.");
}

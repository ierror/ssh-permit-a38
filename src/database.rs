extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub hosts: Vec<Host>,
    pub users: Vec<User>,
    pub groups: Vec<Group>,

    pub last_modified: String,
}

impl Default for Database {
    fn default() -> Database {
        Database {
            hosts: vec![],
            users: vec![],
            groups: vec![],
            last_modified: "".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Host {
    pub hostname: String,
    pub authorized: Vec<User>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub public_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub members: Vec<User>,
}

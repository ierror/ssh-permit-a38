* remove user from authorized and groups if user is deleted
* remove group from authorized groups if group is deleted
* unwrap rm
* user list -> with permissions 
* Default "".to_owned() to None?
* to_owned vs. to_string
* .expect("Couldn't read public key.")
* user_id => id?
* DOC: "unknown => ... to get a list of hostnames available"
* DOC: SSH v2 only
* Create authorized file on first sync
* check for sync_todo = True
* tests for host:port sync
* test for user grant / revoke
* naming user_group vs. group
* host.sync_todo = true wieder rein
* EXAMPLES to DOC:

    cargo run -- host urlsmash.403.io add
    cargo run -- host example.com:22 add
    cargo run -- host list
    cargo run -- host example.com:22 remove
* remove user from authorized and groups if user is deleted 
* unwrap rm
* user list -> with permissions 
* Catch JSON parsing errors
* Default "".to_owned() to None?
* to_owned vs. to_string
* Success messages for add / delete
* .expect("Couldn't read public key.")
* user_id => id?
* DOC: "unknown => ... to get a list of hostnames available"
* DOC: SSH v2 only
* Create authorized on first sync

export OPENSSL_ROOT_DIR=/usr/local/opt/openssl
export OPENSSL_LIB_DIR=/usr/local/opt/openssl/lib
export OPENSSL_INCLUDE_DIR=/usr/local/opt/openssl/include

EXAMPLES:

    cargo run -- host urlsmash.403.io add
    cargo run -- host example.com:22 add
    cargo run -- host list
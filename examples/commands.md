Host
====

## new host
cargo run -- host urlsmash.403.io add

## new host, special ssh port
cargo run -- host example.com:2222 add

## list all hosts
cargo run -- host list

## list specific hosts
cargo run -- host urlsmash.403.io list

## remove host
cargo run -- host example.com:2222 remove

User
====

## new user
cargo run -- user obelix@example.com add

## list all users
cargo run -- user list

## list specific user
cargo run -- host obelix@example.com list
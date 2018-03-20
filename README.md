# SSH Permit A38

Central management and deployment for SSH keys

[![Build Status](https://travis-ci.org/ierror/ssh-permit-a38.svg?branch=master)](https://travis-ci.org/ierror/ssh-permit-a38)

[![asciicast](https://asciinema.org/a/GyIB6XZj7Sotp9ZCekaQcLdDa.png)](https://asciinema.org/a/GyIB6XZj7Sotp9ZCekaQcLdDa)

### Features

* Central management of public SSH keys and servers in a simple and readable JSON database  
* Sync authorized users to the servers authorized_keys
* User Groups
* Diff of authorized_keys to sync and the existing one

## Download prebuilt binaries (v0.0.1)

* [macOS](https://github.com/ierror/ssh-permit-a38/releases/download/v0.0.1/ssh-permit-a38-v0.0.1-x86_64-apple-darwin.zip)
* [Linux x86_64](https://github.com/ierror/ssh-permit-a38/releases/download/v0.0.1/ssh-permit-a38-v0.0.1-x86_64-unknown-linux-gnu.zip)

## Build from source

### Prerequisites

* [Rust](https://www.rust-lang.org/)
* [Cargo](https://doc.rust-lang.org/cargo/)
* [OpenSSL](https://www.openssl.org/)

### Build

```
make build
```

## Quickstart

```
ssh-permit-a38 host urlsmash.403.io add
ssh-permit-a38 user obelix add
ssh-permit-a38 user obelix grant urlsmash.403.io 
ssh-permit-a38 sync
```

## Documentation

Run

```
ssh-permit-a38 howto
```

[or online](https://github.com/ierror/ssh-permit-a38/blob/master/examples/commands.md) 

## Running the tests

```
make test
```

### Coding style

We use rustfmt to format the source.

```
make fmt
```

## Contributing

Pull requests welcome! ;) 

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/ierror/ssh-permit-a38/tags). 

## Authors

* **Bernhard Janetzki**

See also the list of [contributors](https://github.com/ierror/ssh-permit-a38/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## FAQ

* [Permit A38 ?](https://www.youtube.com/watch?v=GI5kwSap9Ug) 

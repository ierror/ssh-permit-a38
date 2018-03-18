# SSH Permit A38

Central managment and deployment for SSH keys

[![asciicast](https://asciinema.org/a/6bJAmxhpldwp43pcvC0WRwDqz.png)](https://asciinema.org/a/6bJAmxhpldwp43pcvC0WRwDqz)

### Features

* Central managament of public SSH keys and servers in a simple and readable JSON database  
* Sync authorized user to the servers authorized_keys
* User Groups
* Diff of authorized_keys to sync and the existing one

## Download prebuilt binaries

* [macOS](https://github.com/ierror/ssh-permit-a38/releases/download/untagged-94dd6630270e1c52de39/ssh-permit-a38-v0.0.1-x86_64-apple-darwin.zip)

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

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## FAQ

* [Permit A38 ?](https://www.youtube.com/watch?v=GI5kwSap9Ug) 

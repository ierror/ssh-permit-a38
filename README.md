# SSH Permit A38

Central management and deployment for SSH keys

[![Build Status](https://travis-ci.org/ierror/ssh-permit-a38.svg?branch=master)](https://travis-ci.org/ierror/ssh-permit-a38)

[![asciicast](https://asciinema.org/a/GyIB6XZj7Sotp9ZCekaQcLdDa.png)](https://asciinema.org/a/GyIB6XZj7Sotp9ZCekaQcLdDa)

### Features

* Central management of public SSH keys and servers in a simple and readable JSON database  
* Sync authorized users to the servers authorized_keys
* SSH config support for the sync command connection paramaters (Hostname, User, Port)
* User Groups
* Host aliases
* Diff of authorized_keys to sync and the existing one

## Latest release v0.2.0 - 2018-08-18

### Download prebuilt binaries 

* [Linux x86_64](https://github.com/ierror/ssh-permit-a38/releases/download/v0.1.0/ssh-permit-a38-v0.2.0-x86_64-unknown-linux-gnu.zip)
* [Linux i686](https://github.com/ierror/ssh-permit-a38/releases/download/v0.1.0/ssh-permit-a38-v0.2.0-i686-unknown-linux-gnu.zip)

* [macOS](https://github.com/ierror/ssh-permit-a38/releases/download/v0.1.0/ssh-permit-a38-v0.2.0-x86_64-apple-darwin.zip)

    or you can install [this Homebrew package](http://formulae.brew.sh/formula/ssh-permit-a38):
    ```
    brew install ssh-permit-a38
    ```

[Previous Releases](https://github.com/ierror/ssh-permit-a38/releases)

### Changelog 

## v0.2.0 - 2018-08-18

- Support for SSH config files [#5](https://github.com/ierror/ssh-permit-a38/issues/5)

    If a ssh-permit-a38 hostname or alias matches the ssh configs Host (or Hostname), User, Port and Host information are used for authorized_keys sync connection

- sync command switch -y, --yes-authorized-keys-prompt:  Automatic yes to authorized_keys location prompts


[Previous Changes](https://github.com/ierror/ssh-permit-a38/blob/master/CHANGELOG.md)

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

* **Bernhard Janetzki** [@i_error](https://twitter.com/i_error)

See also the list of [contributors](https://github.com/ierror/ssh-permit-a38/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## FAQ

* [Permit A38 ?](https://www.youtube.com/watch?v=GI5kwSap9Ug) 

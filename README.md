# SSH Permit A38

Central management and deployment for SSH keys

[![Build Status](https://travis-ci.org/ierror/ssh-permit-a38.svg?branch=master)](https://travis-ci.org/ierror/ssh-permit-a38)

[![asciicast](https://asciinema.org/a/GyIB6XZj7Sotp9ZCekaQcLdDa.png)](https://asciinema.org/a/GyIB6XZj7Sotp9ZCekaQcLdDa)

### Features

* Central management of public SSH keys and servers in a simple and readable JSON database  
* Sync authorized users to the servers authorized_keys
* User Groups
* Host aliases
* Diff of authorized_keys to sync and the existing one

## Latest release v0.1.0 - 2018-04-01

### Download prebuilt binaries 

* [Linux x86_64](https://github.com/ierror/ssh-permit-a38/releases/download/v0.1.0/ssh-permit-a38-v0.1.0-x86_64-unknown-linux-gnu.zip)
* [Linux i686](https://github.com/ierror/ssh-permit-a38/releases/download/v0.1.0/ssh-permit-a38-v0.1.0-i686-unknown-linux-gnu.zip)

* [macOS](https://github.com/ierror/ssh-permit-a38/releases/download/v0.1.0/ssh-permit-a38-v0.1.0-x86_64-apple-darwin.zip)

    or you can install [this Homebrew package](http://formulae.brew.sh/formula/ssh-permit-a38):
    ```
    brew install ssh-permit-a38
    ```

[Previous Releases](https://github.com/ierror/ssh-permit-a38/releases)

### Changelog 

- Support for SSH agent authentication [#4](https://github.com/ierror/ssh-permit-a38/issues/4): - Thank you [@kdar](https://github.com/kdar)

- Support for host aliases [#2](https://github.com/ierror/ssh-permit-a38/issues/2): 

    - Set an alias "um" for hostname "urlsmash.403.io" 
    ```
    ssh-permit-a38 host urlsmash.403.io alias um
    ```

    After this point you can use the alias or the hostname for all host related commands.

    - Remove an alias for hostname "urlsmash.403.io" 
    ```
    ssh-permit-a38 host urlsmash.403.io alias
    ```
    
- Vagrant files and Makefile targets to build linux releases

- Fixed Typos [#1](https://github.com/ierror/ssh-permit-a38/issues/1) [#3](https://github.com/ierror/ssh-permit-a38/issues/3) - Thank you [@0xflotus](https://github.com/0xflotus) and [@robwetz](https://github.com/robwetz)

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

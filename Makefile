.PHONY: build test
SHELL := /bin/sh

# Linux
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    UNAME_P := $(shell uname -p)
    ifeq ($(UNAME_P),x86_64)
        TARGET := x86_64-unknown-linux-gnu
    else
        TARGET := i686-unknown-linux-gnu
    endif
endif

# OS X
ifeq ($(UNAME_S),Darwin)
    TARGET := x86_64-apple-darwin

    # use brew OpenSSL on Mac OSX
    export OPENSSL_ROOT_DIR = /usr/local/opt/openssl
    export OPENSSL_LIB_DIR = /usr/local/opt/openssl/lib
    export OPENSSL_INCLUDE_DIR = /usr/local/opt/openssl/include
endif

# move command line args to RUN_ARGS for the run command
ifeq (run,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "run"
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(RUN_ARGS):;@:)
endif

export RUST_BACKTRACE = 1

fmt:
	cargo fmt

pre_release:
	sed '/^```$$/d;' examples/commands.md > examples/commands.txt

run:
	cargo run -- $(RUN_ARGS)

clean:
	cargo clean

build:
	rustup update stable
	cargo build --release --target=$(TARGET)

build_linux_x86_64:
	cd build && \
	vagrant up linux_x86_64 --provision && \
	vagrant ssh -c "cd /src && make build;" linux_x86_64 && \
	vagrant halt linux_x86_64
	
build_linux_i686:
	cd build && \
	vagrant up linux_i686 --provision && \
	vagrant ssh -c "cd /src && make build;" linux_i686 && \
	vagrant halt linux_i686

release: clean pre_release
	sed -ibak 's/^version = ".*"$$/version = "$(VERSION)"/' Cargo.toml

	# update release urls
	sed -ibak 's/releases\/download\/v.*?\/ssh-permit-a38-v.*?-/releases\/download\/v$(VERSION)\/ssh-permit-a38-v$(VERSION)-/' README.md
	# update release version an date
	sed -ibak 's/^## Latest release v.*/## Latest release v$(VERSION) - $(shell date +%Y-%m-%d)/' README.md

	rm README.mdbak
	rm Cargo.tomlbak

	git commit -a -m "bump $(VERSION)"
	git push
	git checkout master
	git merge develop
	git push origin master
	git tag v$(VERSION)
	git push origin v$(VERSION)
	rm build/binaries/*.zip || true

	# OS X
	cp target/x86_64-apple-darwin/release/ssh-permit-a38 build/binaries/
	cd build/binaries/ && zip --move ssh-permit-a38-v$(VERSION)-x86_64-apple-darwin.zip ssh-permit-a38

	# Linux x86_64
	cp target/x86_64-unknown-linux-gnu/release/ssh-permit-a38 build/binaries/
	cd build/binaries/ && zip --move ssh-permit-a38-v$(VERSION)-x86_64-unknown-linux-gnu.zip ssh-permit-a38

	# Linux i686
	cp target/i686-unknown-linux-gnu/release/ssh-permit-a38 build/binaries/
	cd build/binaries/ && zip --move ssh-permit-a38-v$(VERSION)-i686-unknown-linux-gnu.zip ssh-permit-a38

	cd build/binaries/ && hub release create -a ssh-permit-a38-v$(VERSION)-x86_64-apple-darwin.zip -a ssh-permit-a38-v$(VERSION)-x86_64-unknown-linux-gnu.zip -a ssh-permit-a38-v$(VERSION)-i686-unknown-linux-gnu.zip v$(VERSION)

	open https://github.com/ierror/ssh-permit-a38/releases

test:
	 cargo test --jobs=4 -- --test-threads=4

push: fmt
	git push


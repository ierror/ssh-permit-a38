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

pre_compile:
	sed '/^```$$/d;' examples/commands.md > examples/commands.txt

run:
	cargo run -- $(RUN_ARGS)

clean:
	cargo clean

build:
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

release: pre_compile build build_linux_i686 build_linux_x86_64

test:
	 cargo test --jobs=4 -- --test-threads=4

push: fmt
	git push


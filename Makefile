SHELL := /bin/sh

# use brew OpenSSL on Mac OSX
ifeq ($(shell uname -s),Darwin)
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

run: pre_compile
	cargo run -- $(RUN_ARGS)

clean:
	cargo clean

release: pre_compile
	cargo build --release --target=x86_64-apple-darwin

test:
	 cargo test --jobs=4 -- --test-threads=4

push: fmt
	git push


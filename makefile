all: build

TARGET_DIR=dist
BIN_NAME=inksmash

build: test
	cargo build --release
	cp target/release/$(BIN_NAME) dist/$(BIN_NAME)


test:
# 	https://github.com/rust-lang/rust-clippy
# 	if you want the build job to fail when encountering warnings,
#	and also check tests and non-default crate features
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test --release --workspace


install:
	cargo install -f --path ./

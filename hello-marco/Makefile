rust-version:
	@echo "Rust version: $(shell rustc --version)"
	rustc --version						#rust compiler version
	cargo --version						#rust package manager version
	rustfmt --version					#rust code formatter version
	rustup --version					#rust toolchain installer version
	clippy-driver --version				#rust linter version

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run --quiet

build:
	cargo build --quiet
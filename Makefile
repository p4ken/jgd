.PHONY: all
all:
	cargo test --all-features
	cargo test --no-default-features
	cargo test --release

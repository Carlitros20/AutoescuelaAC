.PHONY: install check test

install:
	cargo fetch

check:
	cargo check

test:
	cargo test

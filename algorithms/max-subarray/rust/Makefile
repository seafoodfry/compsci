.PHONY: all
all: fmt
	cargo test

.PHONY: fmt
fmt:
	rustfmt -l src/*.rs
	cargo clippy

.PHONY: clean
clean:
	cargo clean
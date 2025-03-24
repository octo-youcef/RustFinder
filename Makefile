
.PHONY: install

install:
	cargo build --release --manifest-path finder/Cargo.toml
	cargo install --path finder/

.PHONY: test

test:
	cargo test \
		--verbose \
		--all \
		--manifest-path finder/Cargo.toml

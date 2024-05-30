# VARIABLES
RUST_TOML_DIR := ./rust_tif/Cargo.toml


run-rust:
	cargo run --manifest-path $(RUST_TOML_DIR)

test-rust:
	cargo test --manifest-path $(RUST_TOML_DIR)

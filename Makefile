# VARIABLES
RUST_TOML_DIR := ./rust_tif/Cargo.toml
PYTHON_TOML_DIR := ./python_tif/Cargo.toml


test-rust:
	cargo test --manifest-path $(RUST_TOML_DIR)

test-rusty-py:
	cargo test --manifest-path $(PYTHON_TOML_DIR)

test-py-pytest:
	pytest

install-release:
	pip install -r requirements.txt
	maturin develop -m python_tif/Cargo.toml -r

install-dev:
	pip install -r requirements-dev.txt
	maturin develop -m python_tif/Cargo.toml
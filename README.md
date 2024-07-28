# rust_tif
IO on geospatial files using rust


## Modules

### rust_tif
Pure rust module with pure rust functions. Builds a library.
Run its own tests using cargo test in the rust_tif dir (or use the `makefile`).

### python_tif
Rust-python hybrid library, exposing `rust_tif` functions in a python package.
Run its own (rust) tests using cargo test in the rust_tif dir (or use the `makefile`).

### tests
end-to-end tests to run using pytest in a environment with python_tif installed





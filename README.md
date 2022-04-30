# quep
Quantum performance benchmark tool

## Build
* Need nightly version of the rust toolchain

## Code coverage
* install cargo-tarpaulin
* run linux docker image (e.g. fedora, install rust, python-devel, openssl-devel, gcc)
* run `cargo tarpaulin  --skip-clean --out Html --out Json --out Xml --out Lcov --out Stdout`

# This file is only for development
default: test

test:
    @echo This justfile is only for development
    cargo test --no-default-features -j 1
    cargo test --no-default-features --features updater -j 1
    cargo fmt

build:
    cargo build --no-default-features
    cargo build --no-default-features --features updater

help:
    cargo run --no-default-features -- --help
    cargo run --no-default-features --features updater -- --help

fmt: 
    cargo fmt
    cargo clippy

dev-install:
    cargo install --path . --no-default-features

dev-install-updater:
    cargo install --path . --no-default-features --features updater

bereit: test build fmt

codecov:
    RUSTFLAGS="-Cinstrument-coverage" cargo build 
    LLVM_PROFILE_FILE="your_name-%p-%m.profraw" RUSTFLAGS="-Cinstrument-coverage" cargo test -j 1 --all-features
    grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./codecov
    rm *.profraw

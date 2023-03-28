

default: test

test:
    @echo This justfile is only for development
    cargo test --no-default-features
    cargo test --no-default-features --features updater

build:
    cargo build --no-default-features
    cargo build --no-default-features --features updater

help:
    cargo run --no-default-features -- --help
    cargo run --no-default-features --features updater -- --help
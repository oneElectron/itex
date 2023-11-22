# This file is only for development
set windows-shell := ["powershell.exe"]

default: test

test:
    cargo test --no-default-features --features updater -- --test-threads=1

test-without-features:
    cargo test --no-default-features -- --test-threads=1

test-all: test test-without-features

test-ignored:
    echo This tests all ignored tests
    echo Tests are usually ignored because they require dependacies 
    echo deps: texlive
    cargo test --no-default-features -j 1 -- --include-ignored
    cargo test --no-default-features --features updater -j 1 -- --include-ignored

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
    cargo install --path . --no-default-features --profile dev

dev-install-updater:
    cargo install --path . --no-default-features --features updater --profile dev

bereit: test build fmt

scan:
    trufflehog git https://github.com/oneelectron/itex

codecov:
    rm -rf codecov
    RUSTFLAGS="-Cinstrument-coverage" cargo build --profile codecov
    LLVM_PROFILE_FILE="your_name-%p-%m.profraw" RUSTFLAGS="-Cinstrument-coverage" cargo test -j 1 --all-features --profile codecov
    grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./codecov
    rm *.profraw

fmt:
    cargo fmt --check
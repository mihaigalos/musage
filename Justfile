_default:
  @just --list --unsorted

tool := "musage"

build:
    cargo build

test: build
    cargo test  --verbose --all
    ./target/debug/{{ tool }} --debug
    free -h
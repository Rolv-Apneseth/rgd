alias c := check
alias f := format
alias t := test
alias b := build
alias d := develop
alias r := run
alias rr := run-release
alias p := publish
alias g := gif
alias dg := develop-gif

# COMMANDS -----------------------------------------------------------------------------------------

# List commands
default:
    @just --list

# Check
check:
    cargo check && cargo clippy --all -- -W clippy::all

# Format
format:
    cargo +nightly fmt --all

# Test
test: check format
    cargo test --all
    cargo msrv verify
    cargo deny check

# Build
build: test
    cargo build --release

# Re-run tests on any change
develop: format
    bacon test

# Run the program with args
run *FLAGS:
    cargo run -- {{ FLAGS }}

# Run the program with args - in release mode
run-release *FLAGS:
    cargo run --release -- {{ FLAGS }}

# Publish the crate
publish: test
    cargo publish

# Generate the demo GIF
gif:
    vhs assets/demo.tape --output assets/demo.gif

# Re-generate the demo GIF whenever `demo.tape` is modified
develop-gif:
    echo assets/demo.tape | entr vhs /_ --output assets/demo.gif

[![CI](https://github.com/christopherkeim/rust-cli-template/actions/workflows/ci.yaml/badge.svg)](https://github.com/christopherkeim/rust-cli-template/actions/workflows/ci.yaml)
![Cargo Version](https://img.shields.io/badge/cargo-1.76.0-red.svg)
![Clap](https://img.shields.io/badge/clap-4.5.1-red.svg)

# rust-cli-template

This repository implements a simple Rust command-line interface application that converts webpages to markdown files.

# Quickstart 🦀 🚀 ✨

This repository is a GitHub Template that you can use to create a new repository for Rust-based command-line applications It comes pre-configured to use `cargo 1.76.0` with automated setup for `Ubuntu 20.04/22.04`.

To get started, you can:

1. Simply click `Use this template` in the top right corner of this repository, or

2. Clone this repository onto your machine with:

```bash
git clone https://github.com/christopherkeim/rust-cli-template.git
```

# Setup

**Note: this template targets Ubuntu 20.04/22.04 development environments for automated setup.**

1. Once you have local copy of this repository in your development environment, navigate into this directory and run the `setup.sh` script:

```bash
cd rust-cli-template/
bash setup.sh
```

If you would like to include `Docker 24.0.6` in your development environment, you can run:

```bash
bash setup.sh --docker
```

2. You can ensure that the code currently passes the provided tests with:

```bash
make test
```

3. You can directly try the CLI application with:

```bash
cargo run -- \
  --url https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html \
  --file rust_error_handling.md
```

> This webpage is an excellent resource on error handling rust 😊.

# Development Workflow Automation (`Makefile`)

I like to use a `Makefile` to automate parts of my development workflow - this allows you to alias parts of your development process like linting, formatting, testing, and building binaries.

The `Makefile` in this repository comes with the following commands:

1. `make build`

```bash
cargo build --release
```

2. `make clean`

```bash
cargo clean
```

3. `make doc`

```bash
cargo doc --no-deps --open
```

4. `make lint`

```bash
cargo clippy
```

5. `make format`

```bash
cargo fmt
```

6. `make test`

```bash
cargo test
```

# Continuous Integration (`ci.yaml`)

This repository has continuous integration that runs when Pull Requests are opened to the `main` branch.

**Note that GitHub Action runners come pre-installed with cargo 1.76.0 (latest)**

1. Checks out the source code into the runner VM

2. Formats source code

3. Tests source code

4. Builds a release binary for deployment (commented out - replace with your deployment strategy)

# Containerization (`Dockerfile`)

This template includes a `Dockerfile` that implements a multistage build with the final image using `debian:slim` with the `rust_cli_template` release binary as its entrypoint.

> The `setup.sh` script is _idempotent_, and it will only make changes that bring your development environment closer to the desired state described in the script. If you'd like to include `docker` in your development environment, you can run it again with `--docker`.

If you'd like to containerize the application:

1. Create the image:

```bash
docker build -t rust-cli-template:v0 .
```

2. Run the container with:

```bash
docker run -it rust-cli-template:v0 \
  --url https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html \
  --file rust_error_handling.md
```

For this example application, it's a bit silly. But I've included it as a convenient starting point for building cloud workloads.

# Notes

## Dev and Release Profiles

Cargo has two main profiles: the `dev` profile Cargo uses when you run `cargo build` and the `release` profile Cargo uses when you run `cargo build --release`. The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.

These profile names might be familiar from the output of your builds:

```bash
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
The dev and release are these different profiles used by the compiler.
```

## `opt-level` (Cargo.toml)

By adding `[profile.*]` sections for any profile you want to customize, you override any subset of the default settings. For example, here are the default values for the `opt-level` setting for the `dev` and `release` profiles:

Filename: Cargo.toml

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

The `opt-level` setting controls the number of optimizations Rust will apply to your code, with a range of `0` to `3`. Applying more optimizations extends compiling time, so if you’re in development and compiling your code often, you’ll want fewer optimizations to compile faster even if the resulting code runs slower.

## Documentation (Overall Rust)

The best documentation with very clear examples is Rust By Example: https://doc.rust-lang.org/rust-by-example/index.html

## rustc Targets

`rustc` is a cross-compiler by default. This means that you can use any compiler to build for any architecture. The list of targets are the possible architectures that you can build for can be seen with:

```bash
rustc --print target-list
```

**The rustc Book**: https://doc.rust-lang.org/rustc/targets/index.html

## Cross-compiling for Raspberry Pi Targets

https://jakewharton.com/cross-compiling-static-rust-binaries-in-docker-for-raspberry-pi/

## The Cargo Book

Documentation on Rust's package manager `cargo`.

https://doc.rust-lang.org/cargo/index.html

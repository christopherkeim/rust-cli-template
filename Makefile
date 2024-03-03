build:
	cargo build --release

clean:
	cargo clean

doc:
	cargo doc --no-deps --open

lint:
	cargo clippy

format:
	cargo fmt

test:
	cargo test
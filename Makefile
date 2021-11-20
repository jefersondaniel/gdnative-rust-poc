linux_debug:
	cargo build --target x86_64-unknown-linux-gnu
	RUST_BACKTRACE=1 godot

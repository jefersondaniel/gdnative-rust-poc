linux_debug:
	cargo build --target x86_64-unknown-linux-gnu
	RUST_BACKTRACE=1 /home/jeferson/Projects/Github/godot-34-debug

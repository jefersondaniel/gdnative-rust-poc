linux_debug:
	cargo build --target x86_64-unknown-linux-gnu
	RUST_BACKTRACE=1 godot
wasm_debug:
	RUSTFLAGS="-C link-args=-fPIC -C relocation-model=pic -C target-feature=+mutable-globals" EMMAKEN_CFLAGS="-s SIDE_MODULE=1 -shared -Os" C_INCLUDE_PATH="/home/jeferson/emsdk/upstream/emscripten/cache/sysroot/include/" CARGO_PROFILE_RELEASE_OPT_LEVEL="s" CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS="false" CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS="false" CARGO_PROFILE_RELEASE_PANIC="abort" CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_LINKER="./emcc-test" cargo build --target wasm32-unknown-emscripten

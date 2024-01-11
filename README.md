# GDNative Rust POC

This is a POC of using Rust as a GDNative library in order to build a mugen based game. This POC aims to use a different architecture based on [Bevy ECS](https://bevyengine.org/) and [Rust game + Godot I/O layer](https://godot-rust.github.io/gdnative-book/overview/architecture.html#3-rust-game--godot-io-layer).

## Requirements

* Godot 3.3
* Clang
* Rust

## Setup

1. Clone the repository

```sh
git clone git@github.com:jefersondaniel/gdnative-rust-poc.git
```

2. Download game data

```sh
wget https://github.com/jefersondaniel/godot-mugen-data/archive/refs/tags/1.0.0.zip -O mugen-data.zip
unzip mugen-data.zip && rm mugen-data.zip
```

3. Compile GDNative module

```sh
cargo build --target x86_64-unknown-linux-gnu
```

4. Run Godot (Only tested with Godot 3.3)

```sh
godot
```

## References

I would like to acknowledge the following open-source projects for their contributions and inspiration in the development of this project:

[xnaMugen](https://github.com/scemino/xnamugen): Fork of xnaMugen (MUGEN clone) adapted to Monogame. This project code was used as reference for this implementation.

[Ikemen-Go](https://github.com/ikemen-engine/Ikemen-GO) An open-source fighting game engine that supports MUGEN resources.

# Stratego Game Engine

The main library is written in Rust, but we provide a easy access to the web with TS.

## Python AIs 

Compile Rust sources for Python, you can use `build_python.sh`.

**In game-engine folder**
```shell
cargo build --release

cp target/release/libgame_engine.so ai-python
mv ai-python/libgame_engine.so ai-python/stratego_engine.so
```

You should put all AI files in [ai-python](https://github.com/pr0gen/stratego/tree/python-binding/game-engine/ai-python) folder. Please, make sure this file **compile**, or Rust will throw an uncomprehensible error. I have searched for almost 3hours. :(
Your AIs should depend on interface `class StrategoAI(abc.ABC):`

This function is needed for Rust to call your AI care with the name of this one.
e.g.: ask_next_move_ + ai name (ask_next_move_test)
`def ask_next_move_test() -> Tuple[Tuple[int, str], Tuple[int, str]]:`

The Rust interface is located [there](https://github.com/pr0gen/stratego/blob/python-binding/game-engine/src/player/ai_player.rs)
it will ask your AI if everything is correctly set up his next move.

[This file](https://github.com/pr0gen/stratego/blob/python-binding/game-engine/src/py_bindings.rs) controls Python using Rust code, to move pieces, ask for board state or available moves.

Build only rust 

`
cargo build
cargo test
cargo run
`
You can now play !

## Web interface (In Progress)
Build with webpack 
`wasm-pack build`

# TS version
To run 
`npm rum serve`
### old ts sources :
`npm test` still works


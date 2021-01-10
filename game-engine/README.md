# Stratego Game Engine

The main library is written in Rust. The AI part is using Python.

Please, I'm new to Rust, and we are studiants who are learning. Feel free to make review or reach us if you see garbage code, I would be happy to learn !

## TODO
- [] Maybe a MakeFile to build properly the project
- [] Rust should call the class function instead of a poorly named function
- [] Implementing warp for client REST calls
- [] Return captures as available moves

## Python AIs 

### dependencies 
We used this amazing crate [pyo3](https://github.com/PyO3/pyo3) for python and rust to easily commute, please make sure you can use it before.

**In game-engine folder**

Compile Rust sources for Python, you can use `build_python.sh`.

or 

```shell
cargo build --release
cp target/release/stratego-exec game-stratego
cp target/release/libstratego_lib.so ai-python
mv ai-python/libstratego_lib.so ai-python/stratego_engine.so
```

You should put all AI files in [ai_python](https://github.com/pr0gen/stratego/tree/develop/game-engine/ai_python) folder. Please, make sure this file **compile**, or Rust will throw an uncomprehensible error. I have searched for almost 3hours. :(

Your AIs should depend on interface `class StrategoAI(abc.ABC):`


This function is needed for Rust to call your AI care with the name of this one.
e.g.: ask_next_move_ + ai name (ask_next_move_test)
`def ask_next_move_test() -> Tuple[Tuple[int, str], Tuple[int, str]]:`

The Rust interface is located [there](https://github.com/pr0gen/stratego/tree/develop/game-engine/stratego-lib/src/player/ai_player.rs)
it will ask your AI if everything is correctly set up his next move.

[This file](https://github.com/pr0gen/stratego/tree/develop/game-engine/stratego-lib/src/py_bindings/mod.rs) controls Python using Rust code, to move pieces, ask for board state or available moves.


## How to play 

In stratego-exec, the main function defines a game with a AI.
```rust
    let engine = StrategoEngine::new(
        create_stratego_board(),
        (
            HumanPlayer::new(Color::Red, String::from("Tigran")),
            //HumanPlayer::new(Color::Blue, String::from("Cassiopee")),
            AIPlayer::new(Color::Blue, String::from("test")),
        ),
    );
```
The engine controls board and players. He asks them to enter position to play. (e.g: 0A)

The GamePool is only here for AI to be able to create games and simulates some moves, you can disable it if you are playing PvP.


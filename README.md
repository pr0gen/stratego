# Stratego Game Engine
The main library is written in Rust. The AI part is using Python.
Please, I'm new to Rust, and we are studiants who are learning. Feel free to make review or reach us if you see garbage code, I would be happy to learn !

## TODO
- [ ] Evolve Python api to Rust api
- [ ] Implementing warp for client REST calls

## Python AIs 

### dependencies

We use this amazing crate [pyo3](https://github.com/PyO3/pyo3) for python and rust to easily commute, please make sure you can use it before.

Install python dependencies with `pip3 install -r requirements.txt`

Build engine project

```bash
  make lib-build
```

You should put all AI files in [ai_python](https://github.com/pr0gen/stratego/tree/develop/ai_python/src) folder. Please, make sure this file **compile**, or Rust will throw an uncomprehensible error. I have searched for almost 3hours. :(

Your AIs should depend on interface `class StrategoAI(abc.ABC):`


In [rust_bind](https://github.com/pr0gen/stratego/tree/develop/rust_bind.py), thoses functions are needed for Rust to call your AI care with the name of this one.
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

## How to use Stratego Library

The object `StrategoBoardWrapper` is available to be used in Python. 


```python
    import stratego_engine as se

    board = se.StrategoBoardWrapper(cases) #constructor

    board.moving((1, "A"), (2, "A")) # move a case
    
    board.display_by_color("Blue") 

    board.at((1, "A"))
    board.place("Full", (1, "A"), 1, "Red") # place a piece at coordinate

    board.get_available_moves()
    board.get_available_moves_by_color('Red')
    # maybe parsed in python with parse_moves() in ai.py file 
    board.rust_basic_evaluation() # return color of the winner, if one there is *Yoda*

    board.place("Full", (1, "A"), 1, "Red") # place a piece at coordinate

    board.rust_basic_evaluation() # return color of the winner, if one there is *Yoda*

```

The others three, are made to be displayed `Case`, `Piece`, `Coordinate`

```python
    case.py_get_state()
    case.py_get_content()

```

The following function are there to help you build content for the board: 

```python
    import stratego_engine as se

    se.rust_create_stratego_board() # to get a stratego board randomlly filed

    # you can also build an empty board and fill it yourself

    se.rust_create_empty_stratego_board() 

    piece = se.rust_create_piece(1, "Blue") # 1 correspond to the number in Rust enum

    case = se.rust_create_full_case((1, "A"), piece)
    se.rust_create_empty_case((1, "A")) # without content
    se.rust_create_unreachable_case((1, "A")) # water

```

### Web API

Run web api, with `make api-build`, the app should run on [localhost](http://127.0.0.1:8000)

Found swagger on [localhost](http://127.0.0.1:8000/docs)

Routes: 

- Create a game with 2 players (p1 is red)

  ```
  http://127.0.0.1:8000/create-game
  {
      "player_id_1": "Tigran",
      "player_id_2": "random",
      "type": "h",
      "board": [
          [
              "your initilized board"
          ]
      ]
  }
  ```
- Get state of a game 

  ```
  http://127.0.0.1:8000/game/{uuid}]/{color}
  
  
  ```

- Get all available moves on a game 
  ```
  http://127.0.0.1:8000/moves/{color}/{uuid}

  ```

- Move a piece on a game
 ``` 
 http://127.0.0.1:8000/moves
  {
    "uuid": "uuid",
    "player_color": "Red",
    "coordinate_from": [
        6,
        "A"
    ],
    "coordinate_to": [
        5,
        "A"
    ]
  }

 ```

- Use an AI as a player 
  ```
  http://127.0.0.1:8000/ai
  {
    "uuid": "uuid",
    "color": "Red",
    "ai_name": "random"
  }

   ```



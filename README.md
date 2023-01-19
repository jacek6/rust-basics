# rust-basics
Basic Rust application for self-education-purposes.

## operations1

One of basics application just to practice Rust.

Location: folder `operations1`

How to run:
```
cargo run
```

## oop1

One of basics application just to practice:
 - traits,
 - structs,
 - providing trait as arg/return value.

Location: folder `oop1`

How to run:
```
cargo oop1
```

## tictactoe1

One of basics application just to practice Rust.

Location: folder `tictactoe1`

How to run:
```
cargo run
```

Files:
```
└── tictactoe1
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        └── main.rs
```

Example gameplay:
```
Welcome in tic tac toe game.

Please choose type of player: player 1
1. Human Player
2. Computer Player who chooses first empty field
aaaa
Please provide valid number

Please choose type of player: player 1
1. Human Player
2. Computer Player who chooses first empty field
3
Please choose number in range [1, 2]

Please choose type of player: player 1
1. Human Player
2. Computer Player who chooses first empty field
1
You have choosen Human Player for player 1

Please choose type of player: player 2
1. Human Player
2. Computer Player who chooses first empty field
2
You have choosen Computer Player who chooses first empty field for player 2

Now Player 1 makes a move.

Dear human, please make a move for 'O'
with that game state:
1 2 3
4 5 6
7 8 9
position to place O:
1
Board after move of Player 1:
O - -
- - -
- - -

Now Player 2 makes a move.
FirstItemPlayer is going to make a move: X on position: 2
Board after move of Player 2:
O X -
- - -
- - -

Now Player 1 makes a move.

Dear human, please make a move for 'O'
with that game state:
O X 3
4 5 6
7 8 9
position to place O:
5
Board after move of Player 1:
O X -
- O -
- - -

Now Player 2 makes a move.
FirstItemPlayer is going to make a move: X on position: 3
Board after move of Player 2:
O X X
- O -
- - -

Now Player 1 makes a move.

Dear human, please make a move for 'O'
with that game state:
O X X
4 O 6
7 8 9
position to place O:
9
Board after move of Player 1:
O X X
- O -
- - O

Player 1 have win!
Would you like to play again? (y/n):
aaaaa
Please answer with 'y' or 'n'

Would you like to play again? (y/n):
y
Would you like change players? (y/n):
y
Please choose type of player: player 1
1. Human Player
2. Computer Player who chooses first empty field
```

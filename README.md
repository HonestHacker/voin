# Voin

A chess engine written in Rust.

## Features

- [x] Simple score function (material, piece-square tables, pawn structure, mobility, bishop pair bonus, king safety)
- [x] Simple implementation of Negamax algorithm with Alpha-Beta pruning
- [x] Basic UCI interface

## Getting Started

### Installing

As Voin is a UCI-compatible engine, an UCI client, such as [Cute Chess](https://cutechess.com/) is needed to comfortably interact with the program.
Then you should build the engine using the commands:

```
$ git clone https://github.com/HonestHacker/voin.git
$ cd voin
$ cargo build --release 
```

The compiled program will appear in `./target/release/`.

## Authors

Contributors names and contact info:

* [HonestHacker](https://github.com/HonestHacker)
* [ABEC](https://github.com/ABEC-projects)

## Version History

* 0.1.0
    * Initial Release

## License

This project is licensed under the Apache License 2.0 - see the LICENSE file for details

## Acknowledgments

Inspiration, code snippets, etc.
* [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)

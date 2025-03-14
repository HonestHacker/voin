mod search;
mod evaluation;
mod utils;
mod display;

use std::{io};
use shakmaty::{Chess, fen::Fen, CastlingMode, Position, uci::UciMove};
use search::*;

fn print_engine_info() {
    println!("id name Voin");
    println!("id author Kuznetsov Makar");
    println!("uciok");
}

fn main() -> io::Result<()> {
    let mut enabled = true;
    let mut pos = Chess::default();
    while enabled {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() > 0 {
            let cmd: String = tokens[0].to_string();
            match cmd.as_str() {
                "uci" => print_engine_info(),
                "isready" => println!("readyok"),
                "position" => {
                    let board_type = tokens[1];
                    if board_type == "startpos" {
                        pos = Chess::default();
                    } else {
                        let fen: Fen;
                        if tokens.contains(&"moves") {
                            let idx: usize = tokens.iter().position(|&x| x == "moves").unwrap();
                            fen = tokens[2..idx].join(" ").parse().unwrap();
                        } else {
                            fen = tokens[2..].join(" ").parse().unwrap();
                        }
                        pos = fen.into_position(CastlingMode::Standard).unwrap();
                    }
                    if tokens.contains(&"moves") {
                        let idx: usize = tokens.iter().position(|&x| x == "moves").unwrap() + 1;
                        for s in tokens[idx..].iter() {
                            let uci: UciMove = s.parse().unwrap();
                            let m = uci.to_move(&pos).unwrap();
                            pos.play_unchecked(&m);
                        }
                    }
                }
                "go" => {
                    let (best_move, best_score) = find_best_move(&pos);
                    let best_move_uci = best_move.to_uci(CastlingMode::Standard);
                    println!("info score value cp {} pv {}", best_score, best_move_uci);
                    println!("bestmove {}", best_move_uci);
                }
                "quit" => enabled = false,
                "board" => {
                    let board = pos.board();
                    let s = display::display_board(board);
                    println!("{s}");
                },
                _ => println!("Unknown command"),
            }
        }
    }
    Ok(())
}

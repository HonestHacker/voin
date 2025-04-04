mod search;
mod evaluation;
mod utils;
mod score;
mod transposition;

use std::{io};
use shakmaty::{Chess, fen::Fen, CastlingMode, Position, uci::UciMove, Color};
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
                "ucinewgame" => {
                    pos = Chess::default();
                }
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
                    let remaining_time: i32;
                    if tokens.contains(&"movetime") {
                        remaining_time = tokens[tokens.iter().position(|&r| r == "movetime").unwrap() + 1].parse().unwrap();
                    } else if tokens.contains(&"wtime") && tokens.contains(&"btime") {
                        let wtime: i32 = tokens[tokens.iter().position(|&r| r == "wtime").unwrap() + 1].parse().unwrap();
                        let btime: i32 = tokens[tokens.iter().position(|&r| r == "btime").unwrap() + 1].parse().unwrap();
                        remaining_time = match pos.turn() {
                            Color::White => wtime,
                            Color::Black => btime,
                        };
                    } else {
                        remaining_time = 10_000;
                    }
                    let (best_move, _best_score) = find_best_move(&pos, remaining_time);
                    let best_move_uci = best_move.to_uci(CastlingMode::Standard);
                    println!("bestmove {}", best_move_uci);
                }
                "quit" => {
                    enabled = false;
                }
                &_ => {},
            }
        }
    }
    Ok(())
}

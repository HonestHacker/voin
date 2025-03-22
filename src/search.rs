use std::time::{Instant, Duration};

use shakmaty::zobrist::{ZobristHash, Zobrist64};
use shakmaty::{Chess, Position, Move, Outcome, Color, CastlingMode};
use crate::evaluation::*;
use crate::score::Score;
use crate::transposition::{TranspositionTable, NodeType};
//use crate::utils::signum;

fn quiescence_search(pos: &Chess, mut alpha: Score, beta: Score) -> Score {
    // Check for terminal node first
    if pos.is_game_over() {
        return match pos.outcome().unwrap() {
            Outcome::Decisive { winner } => Score::Mate(1).apply_color_factor(winner),
            Outcome::Draw => Score::ZERO,
        };
    }

    // Stand pat score (current position evaluation)
    let stand_pat = calculate_score(pos).apply_color_factor(pos.turn());
    if stand_pat >= beta {
        return beta;
    }
    alpha = alpha.max(stand_pat);

    // Generate and order capture moves using MVV-LVA heuristic
    let mut captures = pos.legal_moves()
        .into_iter()
        .filter(|m| m.is_capture())
        .collect::<Vec<_>>();

    // MVV-LVA ordering: Most Valuable Victim - Least Valuable Aggressor
    captures.sort_by(|a, b| {
        let a_value = get_piece_value(a.capture().unwrap()) * 10 - get_piece_value(a.role());
        let b_value = get_piece_value(b.capture().unwrap()) * 10 - get_piece_value(b.role());
        b_value.cmp(&a_value)
    });

    for mov in captures {
        let mut new_pos = pos.clone();
        new_pos.play_unchecked(&mov);
        let score = -quiescence_search(&new_pos, -beta, -alpha);
        
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }

    alpha
}

fn negamax(
    pos: &Chess,
    depth: i16,
    mut alpha: Score,
    beta: Score,
    transposition_table: &mut TranspositionTable,
) -> (Score, Option<Move>) {
    let hash = pos.zobrist_hash::<Zobrist64>(shakmaty::EnPassantMode::Legal).into();
    let alpha_orig = alpha;

    if let Some(entry) = transposition_table.get(hash) {
        if entry.depth >= depth {
            match entry.node_type {
                NodeType::Exact => return (entry.score, entry.best_move.clone()),
                NodeType::LowerBound if entry.score >= beta => return (entry.score, entry.best_move.clone()),
                NodeType::UpperBound if entry.score <= alpha => return (entry.score, entry.best_move.clone()),
                _ => {}
            }
        }
    }

    if pos.is_game_over() {
        let eval = calculate_score(pos).apply_color_factor(pos.turn());
        return (eval, None);
    }

    if depth == 0 {
        return (quiescence_search(pos, alpha, beta), None);
    }

    let mut best_value = Score::MIN;
    let mut best_move = None;
    let mut moves = pos.legal_moves();

    moves.sort_by_cached_key(|m| {
        let mut score = Score::ZERO;
        if m.is_capture() {
            score += Score::Centipawn(1000) + get_piece_value(m.capture().unwrap());
        }
        if m.promotion().is_some() {
            score += Score::Centipawn(900);
        }
        -score
    });

    if let Some(entry) = transposition_table.get(hash) {
        if let Some(bm) = &entry.best_move {
            if let Some(idx) = moves.iter().position(|m| m == bm) {
                moves.swap(0, idx);
            }
        }
    }

    let mut first_move = true;
    for mov in moves {
        let mut new_pos = pos.clone();
        new_pos.play_unchecked(&mov);
        let score;

        if first_move {
            let (s, _) = negamax(&new_pos, depth - 1, -beta, -alpha, transposition_table);
            score = -s;
            first_move = false;
        } else {
            let (s_null, _) = negamax(&new_pos, depth - 1, -alpha - 1, -alpha, transposition_table);
            let null_score = -s_null;
            if null_score > alpha {
                let (s_research, _) = negamax(&new_pos, depth - 1, -beta, -alpha, transposition_table);
                score = -s_research;
            } else {
                score = null_score;
            }
        }

        if score > best_value {
            best_value = score;
            best_move = Some(mov.clone());
            if score > alpha {
                alpha = score;
                if alpha >= beta {
                    break;
                }
            }
        }
    }

    let node_type = if best_value <= alpha_orig {
        NodeType::UpperBound
    } else if best_value >= beta {
        NodeType::LowerBound
    } else {
        NodeType::Exact
    };
    transposition_table.insert(hash, depth, best_value, node_type, best_move.clone());

    (best_value, best_move)
}

fn get_principal_variation(pos: &Chess, tt: &TranspositionTable) -> Vec<Move> {
    let mut pv = Vec::new();
    let mut current_pos = pos.clone();
    let mut current_hash = current_pos.zobrist_hash::<Zobrist64>(shakmaty::EnPassantMode::Legal).into();

    loop {
        let entry = tt.get(current_hash);
        if let Some(entry) = entry {
            if let Some(best_move) = &entry.best_move {
                pv.push(best_move.clone());
                current_pos.play_unchecked(best_move);
                current_hash = current_pos.zobrist_hash::<Zobrist64>(shakmaty::EnPassantMode::Legal).into();
            } else {
                break;
            }
        } else {
            break;
        }

        if pv.len() >= 100 {
            break;
        }
    }

    pv
}

pub fn find_best_move(pos: &Chess, remaining_time: i32) -> (Move, Score) {
    let start_time = Instant::now();
    let time_budget = Duration::from_millis(remaining_time as u64 / 40);
    let mut transposition_table = TranspositionTable::new(1 << 20);
    let mut best_move = None;
    let mut best_score = Score::MIN;
    let mut current_depth = 1;

    while current_depth <= 50 {
        let window = if current_depth >= 2 { Score::Centipawn(100) } else { Score::Centipawn(1000) };
        let mut alpha = best_score - window;
        let mut beta = best_score + window;

        if current_depth == 1 {
            alpha = -Score::MAX / 2;
            beta = Score::MAX / 2;
        }

        let (mut score, mut mv) = negamax(pos, current_depth, alpha, beta, &mut transposition_table);

        if score <= alpha {
            (score, mv) = negamax(pos, current_depth, -Score::MAX / 2, beta, &mut transposition_table);
        } else if score >= beta {
            (score, mv) = negamax(pos, current_depth, alpha, Score::MAX / 2, &mut transposition_table);
        }

        if let Some(m) = mv {
            best_move = Some(m);
            best_score = score;
        }

        let pv = get_principal_variation(pos, &transposition_table);
        let pv_uci: Vec<String> = pv.iter()
            .map(|m| m.to_uci(CastlingMode::Standard).to_string())
            .collect();
        let pv_str = pv_uci.join(" ");

        println!(
            "info depth {} score {} time {} pv {}",
            current_depth,
            best_score,
            start_time.elapsed().as_millis(),
            pv_str
        );

        if start_time.elapsed() > time_budget {
            break;
        }
        current_depth += 1;
    }

    (best_move.expect("No legal moves"), best_score)
}
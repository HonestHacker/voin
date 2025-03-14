use shakmaty::{Chess, Position, Move, Outcome};
use crate::evaluation::*;
use crate::utils::*;

fn quiescence_search(pos: &Chess, mut alpha: i16, beta: i16) -> i16 {
    // Check for terminal node first
    if pos.is_game_over() {
        return match pos.outcome().unwrap() {
            Outcome::Decisive { winner } => i16::MAX / 2 * get_color_factor(winner),
            Outcome::Draw => 0,
        };
    }

    // Stand pat score (current position evaluation)
    let stand_pat = calculate_score(pos) * get_color_factor(pos.turn());
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

fn negamax(pos: &Chess, depth: i16, mut alpha: i16, beta: i16) -> (i16, Option<Move>) {
    if pos.is_game_over() {
        let eval = calculate_score(pos) * get_color_factor(pos.turn());
        return (eval, None);
    }

    // At leaf nodes, enter quiescence search
    if depth == 0 {
        return (quiescence_search(pos, alpha, beta), None);
    }

    let mut best_value = i16::MIN;
    let mut best_move = None;

    // Generate and order all legal moves
    let mut moves = pos.legal_moves();
    moves.sort_by_cached_key(|m| {
        // Simple move ordering: prioritize captures and promotions
        let mut score = 0;
        if m.is_capture() {
            score += 1000 + get_piece_value(m.capture().unwrap());
        }
        if m.promotion().is_some() {
            score += 900; // Queen promotion value
        }
        -score // Sort descending
    });

    for mov in moves {
        let mut new_pos = pos.clone();
        new_pos.play_unchecked(&mov);
        let (score, _) = negamax(&new_pos, depth - 1, -beta, -alpha);
        let current_score = -score;

        if current_score > best_value {
            best_value = current_score;
            best_move = Some(mov.clone());
        }

        if current_score > alpha {
            alpha = current_score;
        }

        if alpha >= beta {
            break;
        }
    }

    (best_value, best_move)
}


pub fn find_best_move(pos: &Chess) -> (Move, i16) {
    let depth = 4;
    let (score, best_move) = negamax(pos, depth, -i16::MAX / 2, i16::MAX / 2);
    (best_move.expect("No legal moves available"), score * get_color_factor(pos.turn()))
}
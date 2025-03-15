use shakmaty::{Chess, Position, Role, Piece, Color, Outcome, Square, File, Board};
use crate::{score::Score, utils::*};

// Pawn tables
const PAWN_MG: [i16; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
    50,  50,  50,  50,  50,  50,  50,  50,
    10,  10,  20,  30,  30,  20,  10,  10,
     5,   5,  10,  25,  25,  10,   5,   5,
     0,   0,   0,  20,  20,   0,   0,   0,
     5,  -5, -10,   0,   0, -10,  -5,   5,
     5,  10,  10, -20, -20,  10,  10,   5,
     0,   0,   0,   0,   0,   0,   0,   0,
];

const PAWN_EG: [i16; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
    80,  80,  80,  80,  80,  80,  80,  80,
    50,  50,  50,  50,  50,  50,  50,  50,
    30,  30,  30,  30,  30,  30,  30,  30,
    20,  20,  20,  20,  20,  20,  20,  20,
    10,  10,  10,  10,  10,  10,  10,  10,
     0,   0,   0,   0,   0,   0,   0,   0,
   -50, -50, -50, -50, -50, -50, -50, -50,
];

// Knight tables
const KNIGHT_MG: [i16; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  20,  25,  25,  20,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,   0,  15,  15,   0,   0, -30,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

const KNIGHT_EG: [i16; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

// Bishop tables
const BISHOP_MG: [i16; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

const BISHOP_EG: [i16; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

// Rook tables
const ROOK_MG: [i16; 64] = [
     0,  0,  0,  5,  5,  0,  0,  0,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     5, 10, 10, 10, 10, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0,
];

const ROOK_EG: [i16; 64] = [
    10, 10, 10, 10, 10, 10, 10, 10,
    15, 20, 20, 20, 20, 20, 20, 15,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
     0,  0,  0,  5,  5,  0,  0,  0,
];

// Queen tables
const QUEEN_MG: [i16; 64] = [
    -20, -10, -10,  -5,  -5, -10, -10, -20,
    -10,   0,   5,   0,   0,   0,   0, -10,
    -10,   5,   5,   5,   5,   5,   0, -10,
      0,   0,   5,   5,   5,   5,   0,  -5,
     -5,   0,   5,   5,   5,   5,   0,  -5,
    -10,   0,   5,   5,   5,   5,   0, -10,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -20, -10, -10,  -5,  -5, -10, -10, -20,
];

const QUEEN_EG: [i16; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20,
    -10,   0,   0,  0,  0,   0,   0, -10,
    -10,   0,   5,  5,  5,   5,   0, -10,
     -5,   0,   5,  5,  5,   5,   0,  -5,
      0,   0,   5,  5,  5,   5,   0,  -5,
    -10,   5,   5,  5,  5,   5,   0, -10,
    -10,   0,   5,  0,  0,   0,   0, -10,
    -20, -10, -10, -5, -5, -10, -10, -20,
];

// King tables
const KING_MG: [i16; 64] = [
     20,  30,  10,   0,   0,  10,  30,  20,
     20,  20,   0,   0,   0,   0,  20,  20,
    -10, -20, -20, -20, -20, -20, -20, -10,
    -20, -30, -30, -40, -40, -30, -30, -20,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
];

const KING_EG: [i16; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50,
    -30, -20, -10,   0,   0, -10, -20, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  30,  40,  40,  30, -10, -30,
    -30, -10,  20,  30,  30,  20, -10, -30,
    -30, -30,   0,   0,   0,   0, -30, -30,
    -50, -30, -30, -30, -30, -30, -30, -50,
];

// Piece values for material evaluation
pub fn get_piece_value(role: Role) -> Score {
    Score::Centipawn(match role {
        Role::Pawn => 100,
        Role::Knight => 325,
        Role::Bishop => 350,
        Role::Rook => 500,
        Role::Queen => 1000,
        Role::King => 10000,
    })
}


// Helper function to get positional bonus with phase interpolation
fn get_positional_bonus(piece: Piece, square: Square, phase: f32) -> Score {
    let (mg_table, eg_table) = match piece.role {
        Role::Pawn => (&PAWN_MG, &PAWN_EG),
        Role::Knight => (&KNIGHT_MG, &KNIGHT_EG),
        Role::Bishop => (&BISHOP_MG, &BISHOP_EG),
        Role::Rook => (&ROOK_MG, &ROOK_EG),
        Role::Queen => (&QUEEN_MG, &QUEEN_EG),
        Role::King => (&KING_MG, &KING_EG),
    };

    let (rank, file) = match piece.color {
        Color::White => (square.rank() as usize, square.file() as usize),
        Color::Black => (7 - square.rank() as usize, square.file() as usize),
    };

    let index: usize = rank * 8 + file;
    
    let mg: i16 = mg_table[index];
    let eg: i16 = eg_table[index];
    
    Score::Centipawn((mg as f32 * phase + eg as f32 * (1.0 - phase)) as i16)
}

pub fn calculate_score(pos: &Chess) -> Score {
    if pos.is_game_over() {
        return match pos.outcome().unwrap() {
            Outcome::Decisive { winner } => (Score::MAX / 2).apply_color_factor(winner),
            Outcome::Draw => Score::ZERO,
        };
    }

    let mut score = Score::ZERO;
    let board = pos.board();
    let game_phase = calculate_game_phase(board);

    // Material and positional evaluation
    for square in board.occupied() {
        let piece = board.piece_at(square).unwrap();
        let value = get_piece_value(piece.role);
        let positional = get_positional_bonus(piece, square, game_phase);
        score += (value + positional).apply_color_factor(piece.color);
    }

    // Pawn structure evaluation
    score += evaluate_pawn_structure(board);

    // Mobility evaluation
    score += evaluate_mobility(pos);

    // Bishop pair bonus
    score += evaluate_bishop_pair(board);

    // King safety
    score += evaluate_king_safety(board, game_phase);

    score
}

// Helper functions implementation
fn calculate_game_phase(board: &Board) -> f32 {
    let mut phase = 24.0;
    phase -= (board.knights().count() + board.bishops().count() + board.rooks().count() * 2 + board.queens().count() * 4) as f32;
    clamp(0.0, phase / 24.0, 1.0)
}

fn evaluate_pawn_structure(board: &shakmaty::Board) -> Score {
    let mut score = 0;
    for color in &[Color::White, Color::Black] {
        let pawns = board.by_piece(Piece { color: *color, role: Role::Pawn });
        let mut files = [0u8; 8];
        
        for sq in pawns {
            files[sq.file() as usize] += 1;
            // Isolated pawn check
            if (sq.file() == File::A || files[(sq.file() as usize).saturating_sub(1)] == 0) &&
               (sq.file() == File::H || files[(sq.file() as usize) + 1] == 0) {
                score += if *color == Color::White { -15 } else { 15 };
            }
        }
        
        // Doubled pawns
        for &count in &files {
            if count > 1 {
                score += -get_color_factor(*color) * 20 * (count as i16 - 1);
            }
        }
    }
    Score::Centipawn(score)
}

fn evaluate_mobility(pos: &Chess) -> Score {
    let mut mobility = 0;
    let color = pos.turn();
    let board = pos.board();
    
    for sq in board.by_color(color) {
        if let Some(piece) = board.piece_at(sq) {
            let attacks = board.attacks_from(sq);
            mobility += (attacks.count() as i16) * match piece.role {
                Role::Knight => 2,
                Role::Bishop => 3,
                Role::Rook => 2,
                Role::Queen => 1,
                _ => 0,
            };
        }
    }
    
    Score::Centipawn(mobility).apply_color_factor(color)
}

fn evaluate_bishop_pair(board: &shakmaty::Board) -> Score {
    let mut score = 0;
    for color in &[Color::White, Color::Black] {
        let bishops = board.by_piece(Piece { color: *color, role: Role::Bishop });
        if bishops.count() >= 2 {
            score += if *color == Color::White { 30 } else { -30 };
        }
    }
    Score::Centipawn(score)
}

fn evaluate_king_safety(board: &shakmaty::Board, phase: f32) -> Score {
    let mut score = 0;
    for color in &[Color::White, Color::Black] {
        if let Some(sq) = board.king_of(*color) {
            let penalty = match sq.file() {
                File::D | File::E => (50.0 * phase) as i16,
                _ => (20.0 * phase) as i16,
            };
            score += if *color == Color::White { -penalty } else { penalty };
        }
    }
    Score::Centipawn(score)
}

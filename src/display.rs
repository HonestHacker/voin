use shakmaty::Board;
use std::str::FromStr;

pub fn display_board(board: &Board) -> String {
    let fen = format!("{board}");
    let split = fen.split('/');
    let mut sboard = String::new();
    sboard.push_str(" ==========\n");
    for (rowi, row) in split.enumerate() {
        sboard.push_str(&format!("{rowi}|"));
        for c in row.chars() {
            if c.is_numeric() {
                (0..u8::from_str(&c.to_string()).unwrap()).for_each(|_| {sboard.push(' ')});
            } else {
                sboard.push(c);
            }
        }
        sboard.push_str("|\n");
    }
    sboard.push_str(" ==========\n");
    sboard.push_str("  ABCDFGHI ");

    sboard
}

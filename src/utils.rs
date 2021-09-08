use super::*;

pub fn generate_backrank(color: Color) -> [Option<Piece>; 8] {
    [
        Some((Rank::Rook, color)),
        Some((Rank::Knight, color)),
        Some((Rank::Bishop, color)),
        Some((Rank::Queen, color)),
        Some((Rank::King, color)),
        Some((Rank::Bishop, color)),
        Some((Rank::Knight, color)),
        Some((Rank::Rook, color)),
    ]
}

// TODO: Is there a simpler way to generate this? Tried using a range but
// was difficult convert a vec to an array.
pub fn generate_pawns(color: Color) -> [Option<Piece>; 8] {
    [
        Some((Rank::Pawn, color)),
        Some((Rank::Pawn, color)),
        Some((Rank::Pawn, color)),
        Some((Rank::Pawn, color)),
        Some((Rank::Pawn, color)),
        Some((Rank::Pawn, color)),
        Some((Rank::Pawn, color)),
        Some((Rank::Pawn, color)),
    ]
}

pub fn generate_empty() -> [Option<Piece>; 8] {
    [None, None, None, None, None, None, None, None]
}

// Translates a position on the board (Option<Piece) into a usize
// for sending to the JS layer
fn get_piece_code(p: Option<Piece>) -> usize {
    match p {
        None => 0,
        Some((r, c)) => {
            match c {
                Color::White => {
                    match r {
                        Rank::Pawn => 1,
                        Rank::Knight => 2,
                        Rank::Bishop => 3,
                        Rank::Rook => 4,
                        Rank::Queen => 5,
                        Rank::King => 6,
                    }
                },
                Color::Black => {
                    match r {
                        Rank::Pawn => 7,
                        Rank::Knight => 8,
                        Rank::Bishop => 9,
                        Rank::Rook => 10,
                        Rank::Queen => 11,
                        Rank::King => 12,
                    }
                }
            }
        },
    }
}

// Converts a MoveResult (which carries basic info about the affected
// pieces on the board) into a simple array of usize
fn serialize_move_result(mr: MoveResult) -> [usize; 8] {
    let success = match mr.success {
        true => 1,
        false => 0,
    };

    let (from_x, from_y) = mr.from_loc;
    let (to_x, to_y) = mr.to_loc;

    let from_piece = get_piece_code(mr.from_piece);
    let to_piece = get_piece_code(mr.to_piece);

    let turn = match mr.turn {
        Color::White => 0,
        Color::Black => 1,
    };

    [success, from_x, from_y, from_piece, to_x, to_y, to_piece, turn]
}


#[wasm_bindgen]
#[allow(dead_code)]
pub fn client_msg(player: Color, from_x: usize, from_y: usize, to_x: usize, to_y: usize) ->Box<[usize]> {
    let move_result = GAME.lock().unwrap().try_move(player, (from_x, from_y), (to_x, to_y));
    Box::new(serialize_move_result(move_result))
}

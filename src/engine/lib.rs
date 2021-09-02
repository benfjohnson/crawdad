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

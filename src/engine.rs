use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
mod utils;

// This would be sketchy if we were using it in a threaded context,
// but WebAssembly doesn't have threads
// TODO: Move this out of core engine code
lazy_static! {
 static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rank {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

type Piece = (Rank, Color);

pub struct MoveResult {
    turn: Color,
    from_loc: (usize, usize),
    to_loc: (usize, usize),
    from_piece: Option<Piece>,
    to_piece: Option<Piece>,
    success: bool,
}

type Board = [[Option<Piece>; 8]; 8];

pub struct Game {
    turn: Color,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn: Color::White,
            board: [
                utils::generate_backrank(Color::White),
                utils::generate_pawns(Color::White),
                utils::generate_empty(),
                utils::generate_empty(),
                utils::generate_empty(),
                utils::generate_empty(),
                utils::generate_pawns(Color::Black),
                utils::generate_backrank(Color::Black),
            ]
        }
    }

    pub fn try_move(&mut self, color: Color, from: (usize, usize), to: (usize, usize)) -> MoveResult {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;

        let default_move = MoveResult {
            turn: self.turn,
            from_loc: from,
            to_loc: to,
            from_piece: self.board[from_y][from_x],
            to_piece: self.board[to_y][to_x],
            success: false,
        };

        // Must be the turn of user attemping to move
        if color != self.turn {
            return default_move;
        }

        // Cannot initiate a move from an empty square, or a square with opponent piece
        match self.board[from_y][from_x] {
            None => {
                return default_move;
            },
            Some((_, c)) => {
                if color != c { return default_move; }
            } 
        }

        // Cannot move onto a square already occupied by piece of same color
        match self.board[to_y][to_x] {
            None => {},
            Some((_, c)) => {
                if color == c { return default_move; }
            }
        }


        // Valid move. Update game accordingly!
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.board[to_y][to_x] = self.board[from_y][from_x];
        self.board[from_y][from_x] = None;

        MoveResult {
            turn: self.turn,
            from_loc: from,
            to_loc: to,
            from_piece: self.board[from_y][from_x],
            to_piece: self.board[to_y][to_x],
            success: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e4_opening() {
        let mut g = Game::new();
        let move_res = g.try_move(Color::White, (4, 1), (4, 3));
        assert_eq!(move_res.from_piece, None);
        assert_eq!(move_res.to_piece, Some((Rank::Pawn, Color::White)));
    }
}
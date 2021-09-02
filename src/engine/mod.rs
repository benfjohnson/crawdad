mod lib;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum Rank {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

type Piece = (Rank, Color);

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
                lib::generate_backrank(Color::White),
                lib::generate_pawns(Color::White),
                lib::generate_empty(),
                lib::generate_empty(),
                lib::generate_empty(),
                lib::generate_empty(),
                lib::generate_pawns(Color::Black),
                lib::generate_backrank(Color::Black),
            ]
        }
    }

    pub fn try_move(&mut self, color: Color, from: (usize, usize), to: (usize, usize)) -> bool {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;

        // Must be the turn of user attemping to move
        if color != self.turn {
            return false;
        }

        // Cannot initiate a move from an empty square, or a square with opponent piece
        match self.board[from_y][from_x] {
            None => {
                return false;
            },
            Some((_, c)) => {
                if color != c { return false; }
            } 
        }

        // Cannot move onto a square already occupied by piece of same color
        match self.board[to_y][to_x] {
            None => {},
            Some((_, c)) => {
                if color == c { return false; }
            }
        }


        // Valid move. Update game accordingly!
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        self.board[to_y][to_x] = self.board[from_y][from_x];
        self.board[from_y][from_x] = None;

        true
    }
}


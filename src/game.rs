use crate::error::MoveError;
use std::str::FromStr;

pub mod default_piece {
    pub enum Color {
        White,
        Black,
    }

    pub enum Rank {
        Man,
        King,
    }

    pub struct Piece {
        pub color: Color,
        pub rank: Rank,
    }
}

pub struct GameData {
    pub id: &'static str,
    pub board_rows: u8,
    pub board_columns: u8,
}

impl GameData {
    pub fn board_size(&self) -> u8 {
        self.board_rows * self.board_columns
    }

    pub fn valid_squares_count(&self) -> u8 {
        self.board_size() >> 1
    }
}

pub trait Game: Sized {
    type M: Move;
    type P;

    fn make_move(&mut self, mv: Self::M) -> Result<&Self, MoveError<Self::M>>;
}

pub trait Bitboard: FromStr {
    type M: Move;
    type P;

    fn set_piece_at(&mut self, piece: Option<Self::P>, pos: u8);

    fn get_piece_at(&self, pos: u8) -> Option<Self::P>;
}

pub trait Move {
    fn match_string(&self, movestr: &str) -> bool {
        movestr == self.to_string(true).as_str()
    }

    fn to_string(&self, longform: bool) -> String;
}

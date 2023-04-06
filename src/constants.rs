use crate::{Pos};

pub const BOARD_SIZE: usize = 8;
pub const ADJACENT_OFFSETS: [Pos; 8] = [
    Pos {
        x: -1,
        y: -1,
    },
    Pos {
        x: 0,
        y: -1,
    },
    Pos {
        x: 1,
        y: -1,
    },
    Pos {
        x: 1,
        y: 0,
    },
    Pos {
        x: 1,
        y: 1,
    },
    Pos {
        x: 0,
        y: 1,
    },
    Pos {
        x: -1,
        y: 1,
    },
    Pos {
        x: -1,
        y: 0,
    },
];
pub const BISHOP_OFFSETS: [Pos; 4] = [
    Pos {
        x: -1,
        y: -1,
    },
    Pos {
        x: 1,
        y: -1,
    },
    Pos {
        x: 1,
        y: 1,
    },
    Pos {
        x: -1,
        y: 1,
    },
];
pub const CASTLE_OFFSETS: [Pos; 4] = [
    Pos {
        x: 0,
        y: -1,
    },
    Pos {
        x: 1,
        y: 0,
    },
    Pos {
        x: 0,
        y: 1,
    },
    Pos {
        x: -1,
        y: 0,
    },
];
pub const KNIGHT_OFFSETS: [Pos; 8] = [
    // Horizontal
    Pos {
        x: -2,
        y: 1,
    },
    Pos {
        x: 2,
        y: 1,
    },
    Pos {
        x: -2,
        y: -1,
    },
    Pos {
        x: 2,
        y: -1,
    },
    // Vertical
    Pos {
        x: -1,
        y: 2,
    },
    Pos {
        x: 1,
        y: 2,
    },
    Pos {
        x: -1,
        y: -2,
    },
    Pos {
        x: 1,
        y: -2,
    },
];
pub const PAWN_ATTACK_OFFSETS_BLACK: [Pos; 2] = [
    Pos {
        x: -1,
        y: -1,
    },
    Pos {
        x: 1,
        y: -1,
    }
];
pub const PAWN_ATTACK_OFFSETS_WHITE: [Pos; 2] = [
    Pos {
        x: -1,
        y: 1,
    },
    Pos {
        x: 1,
        y: 1,
    }
];
#[derive(Debug)]
pub struct ASCIIGraphics<'a> {
    king: &'a str,
    queen: &'a str,
    bishop: &'a str,
    castle: &'a str,
    knight: &'a str,
    pawn: &'a str,
}
#[derive(Debug)]
pub struct Players<'a> {
    black: ASCIIGraphics<'a>,
    white: ASCIIGraphics<'a>,
}
pub const UNIT_GRAPHICS: Players = Players {
    black: ASCIIGraphics {
        king: "♚",
        queen: "♛",
        bishop: "♝",
        castle: "♜",
        knight: "♞",
        pawn: "♟",
    },
    white: ASCIIGraphics {
        king: "♔",
        queen: "♕",
        bishop: "♗",
        castle: "♖",
        knight: "♘",
        pawn: "♙",
    },
};
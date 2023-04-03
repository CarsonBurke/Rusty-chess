use crate::{Pos};

pub const BOARD_SIZE: usize = 8;
pub const ADJACENT_OFFSETS: Vec<Pos> = vec![
    {
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
pub const BISHOP_OFFSETS: Vec<Pos> = vec![
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
pub const CASTLE_OFFSETS: Vec<Pos> = vec![
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
pub const KNIGHT_OFFSETS: Vec<Pos> = vec![
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
pub const PAWN_ATTACK_OFFSETS_BLACK: Vec<Pos> = vec![
    Pos {
        x: -1,
        y: -1,
    },
    Pos {
        x: 1,
        y: -1,
    }
];
pub const PAWN_ATTACK_OFFSETS_WHITE: Vec<Pos> = vec![
    Pos {
        x: -1,
        y: 1,
    },
    Pos {
        x: 1,
        y: 1,
    }
];
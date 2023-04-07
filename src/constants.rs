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

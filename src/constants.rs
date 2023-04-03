pub const BOARD_SIZE: usize = 8;
pub const ADJACENT_OFFSETS: Vec<Pos> = vec![
    {
        x: -1,
        y: -1,
    },
    {
        x: 0,
        y: -1,
    },
    {
        x: 1,
        y: -1,
    },
    {
        x: 1,
        y: 0,
    },
    {
        x: 1,
        y: 1,
    },
    {
        x: 0,
        y: 1,
    },
    {
        x: -1,
        y: 1,
    },
    {
        x: -1,
        y: 0,
    },
];
pub const BISHOP_OFFSETS: Vec<Pos> = vec![
    {
        x: -1,
        y: -1,
    },
    {
        x: 1,
        y: -1,
    },
    {
        x: 1,
        y: 1,
    },
    {
        x: -1,
        y: 1,
    },
];
pub const CASTLE_OFFSETS: Vec<Pos> = vec![
    {
        x: 0,
        y: -1,
    },
    {
        x: 1,
        y: 0,
    },
    {
        x: 0,
        y: 1,
    },
    {
        x: -1,
        y: 0,
    },
];
pub const KNIGHT_OFFSETS: Vec<Pos> = vec![
    // Horizontal
    {
        x: -2,
        y: 1,
    },
    {
        x: 2,
        y: 1,
    },
    {
        x: -2,
        y: -1,
    },
    {
        x: 2,
        y: -1,
    },
    // Vertical
    {
        x: -1,
        y: 2,
    },
    {
        x: 1,
        y: 2,
    },
    {
        x: -1,
        y: -2,
    },
    {
        x: 1,
        y: -2,
    },
];
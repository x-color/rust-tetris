use block_kind::{I, J, L, O, S, T, Z};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub type BlockColor = usize;

pub mod block_kind {
    pub const NONE: super::BlockColor = 0;
    pub const WALL: super::BlockColor = 1;
    pub const GHOST: super::BlockColor = 2;
    pub const I: super::BlockColor = 3;
    pub const O: super::BlockColor = 4;
    pub const S: super::BlockColor = 5;
    pub const Z: super::BlockColor = 6;
    pub const J: super::BlockColor = 7;
    pub const L: super::BlockColor = 8;
    pub const T: super::BlockColor = 9;
}

pub const COLOR_TABLE: [&str; 10] = [
    "\x1b[48;2;000;000;000m  ", // 何もなし
    "\x1b[48;2;127;127;127m  ", // 壁
    "\x1b[48;2;000;000;000m[]", // ゴースト
    "\x1b[48;2;000;000;255m  ", // I
    "\x1b[48;2;000;255;000m  ", // O
    "\x1b[48;2;000;255;255m  ", // S
    "\x1b[48;2;255;000;000m  ", // Z
    "\x1b[48;2;255;000;255m  ", // J
    "\x1b[48;2;255;127;000m  ", // L
    "\x1b[48;2;255;255;000m  ", // T
];

#[derive(Clone, Copy)]
pub enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

impl Distribution<BlockKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BlockKind {
        match rng.gen_range(0..=6) {
            0 => BlockKind::I,
            1 => BlockKind::O,
            2 => BlockKind::S,
            3 => BlockKind::Z,
            4 => BlockKind::J,
            5 => BlockKind::L,
            _ => BlockKind::T,
        }
    }
}

pub type BlockShape = [[usize; 4]; 4];

pub const BLOCKS: [BlockShape; 7] = [
    // Iブロック
    [[0, 0, 0, 0], [0, 0, 0, 0], [I, I, I, I], [0, 0, 0, 0]],
    // Oブロック
    [[0, 0, 0, 0], [0, O, O, 0], [0, O, O, 0], [0, 0, 0, 0]],
    // Sブロック
    [[0, 0, 0, 0], [0, S, S, 0], [S, S, 0, 0], [0, 0, 0, 0]],
    // Zブロック
    [[0, 0, 0, 0], [Z, Z, 0, 0], [0, Z, Z, 0], [0, 0, 0, 0]],
    // Jブロック
    [[0, 0, 0, 0], [J, 0, 0, 0], [J, J, J, 0], [0, 0, 0, 0]],
    // Lブロック
    [[0, 0, 0, 0], [0, 0, L, 0], [L, L, L, 0], [0, 0, 0, 0]],
    // Tブロック
    [[0, 0, 0, 0], [0, T, 0, 0], [T, T, T, 0], [0, 0, 0, 0]],
];

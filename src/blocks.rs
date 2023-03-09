use block_kind::{I, J, L, O, S, T, Z};
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng, Rng,
};

const BLOCK_KIND_MAX: usize = 7;
#[derive(Clone, Copy)]
pub enum Blockkind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

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
    "\x1b[48;2;127;127;127m__", // 壁
    "\x1b[48;2;000;000;000m[]", // ゴースト
    "\x1b[48;2;000;000;255m__", // I
    "\x1b[48;2;000;255;000m__", // O
    "\x1b[48;2;000;255;255m__", // S
    "\x1b[48;2;255;000;000m__", // Z
    "\x1b[48;2;255;000;255m__", // J
    "\x1b[48;2;255;127;000m__", // L
    "\x1b[48;2;255;255;000m__", // T
];

impl Distribution<Blockkind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Blockkind {
        match rng.gen_range(0..=6) {
            0 => Blockkind::I,
            1 => Blockkind::O,
            2 => Blockkind::S,
            3 => Blockkind::Z,
            4 => Blockkind::J,
            5 => Blockkind::L,
            _ => Blockkind::T,
        }
    }
}

pub type BlockShape = [[usize; 4]; 4];

pub const BLOCKS: [BlockShape; BLOCK_KIND_MAX] = [
    // I block
    [[0, 0, 0, 0], [0, 0, 0, 0], [I, I, I, I], [0, 0, 0, 0]],
    // O block
    [[0, 0, 0, 0], [0, O, O, 0], [0, O, O, 0], [0, 0, 0, 0]],
    // S block
    [[0, 0, 0, 0], [0, S, S, 0], [S, S, 0, 0], [0, 0, 0, 0]],
    // Z block
    [[0, 0, 0, 0], [Z, Z, 0, 0], [0, Z, Z, 0], [0, 0, 0, 0]],
    // J block
    [[0, 0, 0, 0], [J, 0, 0, 0], [J, J, J, 0], [0, 0, 0, 0]],
    // L block
    [[0, 0, 0, 0], [0, 0, L, 0], [L, L, L, 0], [0, 0, 0, 0]],
    // T block
    [[0, 0, 0, 0], [0, T, 0, 0], [T, T, T, 0], [0, 0, 0, 0]],
];

pub fn gen_block_7() -> [BlockShape; BLOCK_KIND_MAX] {
    let mut rng = thread_rng();
    let mut que = [
        Blockkind::I,
        Blockkind::O,
        Blockkind::S,
        Blockkind::Z,
        Blockkind::J,
        Blockkind::L,
        Blockkind::T,
    ];
    que.shuffle(&mut rng);
    que.map(|block| BLOCKS[block as usize])
}

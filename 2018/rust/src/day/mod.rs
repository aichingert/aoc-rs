pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;

pub use d03::no_matter::Claim;
pub use d06::chronal_coordinates::Point;

pub mod wrapper;
pub use wrapper::Wrapper;

use std::collections::{HashMap, VecDeque};

pub enum Input {
    Vi32(Vec<i32>),
    VDu8(VecDeque<u8>),
    VVch(Vec<Vec<char>>),
    D03(Vec<Claim>),
    D04(HashMap<u16, Vec<[u16; 60]>>),
    D06((Vec<Point<i32>>, [i32; 4])),
}

#[derive(Debug)]
pub enum Output {
    None,
    Ni32(i32),
    Nu16(u16),
    Nu32(u32),
    Nusize(usize),
    S(String),
}

pub enum InputError {
    NoInput,
    InvalidInput,
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InputError::NoInput => write!(f, "No input provided"),
            InputError::InvalidInput => write!(f, "Input is invalid"),
        }
    }
}

impl From<std::io::Error> for InputError {
    fn from(_: std::io::Error) -> Self {
        InputError::NoInput
    }
}

impl From<std::num::ParseIntError> for InputError {
    fn from(_: std::num::ParseIntError) -> Self {
        InputError::InvalidInput
    }
}

pub type InputResult<T> = std::result::Result<T, InputError>;

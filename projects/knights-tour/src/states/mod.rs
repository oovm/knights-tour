mod knights;
mod solve_walk;
mod states;

mod solve_tour;

pub use self::knights::KnightsTour;
use crate::{utils::format_point, SvgRender};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
    iter::from_generator,
};

/// The state of the chess walk
#[derive(Clone)]
pub struct ChessWalkState {
    size_x: isize,
    size_y: isize,
    current_x: isize,
    current_y: isize,
    visited: BTreeMap<(isize, isize), bool>,
    path: Vec<(isize, isize)>,
    available_moves: Vec<(isize, isize)>,
}

/// The state of the chess tour, which is a chess walk with a back-to-start constraint
#[derive(Clone)]
pub struct ChessTourState {
    size_x: isize,
    size_y: isize,
    current_x: isize,
    current_y: isize,
    visited: BTreeMap<(isize, isize), bool>,
    path: Vec<(isize, isize)>,
    available_moves: Vec<(isize, isize)>,
}

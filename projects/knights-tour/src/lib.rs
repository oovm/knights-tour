#![feature(generators)]
#![feature(iter_from_generator)]
#![feature(type_alias_impl_trait)]
mod knights;

mod utils;

pub use self::knights::{KnightsTour, KnightsTourState};
mod render;

pub use crate::render::SvgRender;

#![feature(generators)]
#![feature(iter_from_generator)]
#![feature(type_alias_impl_trait)]
mod states;

mod utils;

pub use self::states::ChessTourState;
mod render;

pub use crate::render::SvgRender;
mod roles;
pub use self::roles::{ChessRole, Chessboard};

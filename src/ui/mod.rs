mod backend;
mod preview_area;
mod rect;

pub mod views;
pub mod widgets;

pub use backend::{termion::TermionAppBackend, traits::AppBackend};
pub use preview_area::*;
pub use rect::*;

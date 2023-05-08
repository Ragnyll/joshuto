mod backend;
mod preview_area;
mod rect;

pub mod views;
pub mod widgets;

pub use backend::traits::AppBackend;
#[cfg(feature = "crossterm-backend")]
pub use backend::crossterm::CrosstermAppBackend;
#[cfg(not(feature = "crossterm-backend"))]
pub use backend::termion::TermionAppBackend;
pub use preview_area::*;
pub use rect::*;

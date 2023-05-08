pub mod app_event;
pub mod process_event;
pub mod termion;
pub mod crossterm;

pub use self::termion::*;
pub use self::crossterm::*;
pub use self::process_event::*;

use std::sync::mpsc;
pub trait JoshutoEvent {
    fn new() -> Self;

    // We need a next() and a flush() so we don't continuously consume
    // input from the console. Sometimes, other applications need to
    // read terminal inputs while joshuto is in the background
    fn next(&self) -> Result<AppEvent, mpsc::RecvError>;

    fn flush(&self);

}

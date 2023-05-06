use std::io::{self, stdout, Write};
use tui::backend::CrosstermBackend;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

trait New {
    fn new() -> std::io::Result<Self>
    where
        Self: Sized;
}

#[cfg(not(feature = "mouse"))]
type Screen = AlternateScreen<RawTerminal<std::io::Stdout>>;
#[cfg(not(feature = "mouse"))]
impl New for Screen {
    fn new() -> io::Result<Self> {
        // This may not work the same as termion. termion returns an alternate screen once that
        // context has been entered. Crossterm executes a command to go to the alternate screen
        // using a command.
        let stdout = std::io::stdout();
            //.into_raw_mode()?;
        let alt_screen = AlternateScreen::from(stdout);
        Ok(alt_screen)
    }
}

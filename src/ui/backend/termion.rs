use std::io::{self, stdout, Write};

use super::traits::AppBackend;
use termion::{
    raw::{IntoRawMode, RawTerminal},
    screen::AlternateScreen,
};
use tui::{backend::TermionBackend, widgets::Widget};

#[cfg(feature = "mouse")]
use termion::input::MouseTerminal;

trait New {
    fn new() -> std::io::Result<Self>
    where
        Self: Sized;
}

#[cfg(feature = "mouse")]
type Screen = MouseTerminal<AlternateScreen<RawTerminal<std::io::Stdout>>>;
#[cfg(feature = "mouse")]
impl New for Screen {
    fn new() -> std::io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let alt_screen = MouseTerminal::from(AlternateScreen::from(stdout));
        return Ok(alt_screen);
    }
}
#[cfg(not(feature = "mouse"))]
type Screen = AlternateScreen<RawTerminal<std::io::Stdout>>;
#[cfg(not(feature = "mouse"))]
impl New for Screen {
    fn new() -> io::Result<Self> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let alt_screen = AlternateScreen::from(stdout);
        Ok(alt_screen)
    }
}

pub type TermionTuiTerminal = tui::Terminal<TermionBackend<Screen>>;

pub struct TermionAppBackend {
    pub terminal: Option<TermionTuiTerminal>,
}

impl AppBackend for TermionAppBackend {
    type TuiTerminal = TermionTuiTerminal;

    fn new() -> io::Result<Self> {
        let mut alt_screen = Screen::new()?;
        // clears the screen of artifacts
        write!(alt_screen, "{}", termion::clear::All)?;

        let backend = TermionBackend::new(alt_screen);
        let mut terminal = tui::Terminal::new(backend)?;
        terminal.hide_cursor()?;
        Ok(Self {
            terminal: Some(terminal),
        })
    }

    fn render<W>(&mut self, widget: W)
    where
        W: Widget,
    {
        let _ = self.terminal_mut().draw(|frame| {
            let rect = frame.size();
            frame.render_widget(widget, rect);
        });
    }

    fn terminal(&self) -> Option<Self::TuiTerminal> {
        self.terminal
    }

    fn terminal_ref(&self) -> &Self::TuiTerminal {
        self.terminal.as_ref().unwrap()
    }

    fn terminal_mut(&mut self) -> &mut Self::TuiTerminal {
        self.terminal.as_mut().unwrap()
    }

    fn terminal_drop(&mut self) {
        let _ = self.terminal.take();
        let _ = stdout().flush();
    }

    fn terminal_restore(&mut self) -> io::Result<()> {
        let mut new_backend = Self::new()?;
        std::mem::swap(&mut self.terminal, &mut new_backend.terminal);
        Ok(())
    }
}

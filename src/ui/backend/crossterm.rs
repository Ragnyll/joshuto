use crossterm::{
    execute,
    terminal::EnterAlternateScreen,
};
use std::io::{self, stdout, Write};
use super::traits::AppBackend;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use tui::widgets::Widget;

// TODO: put in mouse features

trait New {
    fn new() -> std::io::Result<Self>
    where
        Self: Sized;
}

pub struct CrosstermAppBackend {
    pub terminal: Option<Terminal<CrosstermBackend<io::Stdout>>>,
}

impl AppBackend for CrosstermAppBackend {
    type TuiTerminal = CrosstermBackend<io::Stdout>;

    fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        Ok(Self {
            terminal: Some(terminal)
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

    fn terminal(&self) -> &Option<Terminal<Self::TuiTerminal>> {
        &self.terminal
    }

    fn terminal_ref(&self) -> &Terminal<Self::TuiTerminal> {
        self.terminal.as_ref().unwrap()
    }

    fn terminal_mut(&mut self) -> &mut Terminal<Self::TuiTerminal> {
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

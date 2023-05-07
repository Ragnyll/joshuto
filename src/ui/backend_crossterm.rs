use crossterm::{
    execute,
    terminal::EnterAlternateScreen,
};
use std::io::{self, stdout, Write};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

use tui::widgets::Widget;

pub type TuiTerminal = tui::Terminal<CrosstermBackend<io::Stdout>>;
pub struct CrosstermAppBackend {
    pub terminal: Option<TuiTerminal>
}

impl CrosstermAppBackend {
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend).unwrap();
        terminal.hide_cursor().unwrap();

        Ok(Self {
            terminal: Some(terminal)
        })
    }

    pub fn render<W>(&mut self, widget: W)
    where
        W: Widget,
    {
        let _ = self.terminal_mut().draw(|frame| {
            let rect = frame.size();
            frame.render_widget(widget, rect);
        });
    }

    pub fn terminal_ref(&self) -> &TuiTerminal {
        self.terminal.as_ref().unwrap()
    }

    pub fn terminal_mut(&mut self) -> &mut TuiTerminal {
        self.terminal.as_mut().unwrap()
    }

    pub fn terminal_drop(&mut self) {
        let _ = self.terminal.take();
        let _ = stdout().flush();
    }

    pub fn terminal_restore(&mut self) -> io::Result<()> {
        let mut new_backend = Self::new()?;
        std::mem::swap(&mut self.terminal, &mut new_backend.terminal);
        Ok(())
    }
}




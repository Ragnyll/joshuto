use std::io;
use tui::{backend::Backend, terminal::Terminal, widgets::Widget};

/// Trait used for terminal backends (termion, crossterm, etc.) to interact with Joshuto
pub trait AppBackend {
    type TuiTerminal: Backend;

    /// Build a new AppBackend
    fn new() -> io::Result<Self>
    where
        Self: Sized;

    /// Terminal backends must be able to register widgets
    fn render<W>(&mut self, widget: W)
    where
        W: Widget;

    fn terminal(&self) -> &Option<Terminal<Self::TuiTerminal>>;

    /// AppBackends must be able to return a ref to its inner backend object
    fn terminal_ref(&self) -> &Terminal<Self::TuiTerminal>;

    /// AppBackends must be able to return a mutable ref to its inner backend object
    fn terminal_mut(&mut self) -> &mut Terminal<Self::TuiTerminal>;

    /// AppBackends must be able to drop their inner terminal.
    fn terminal_drop(&mut self);

    /// AppBackends must be able to restore the terminal to its original state
    fn terminal_restore(&mut self) -> io::Result<()>;
}

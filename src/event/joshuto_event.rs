use std::{fmt::Debug, string::ToString};

/// An internal type used to route various backend event types through. For example termion and
/// crossterm both have their own event types which can change however they want but it must be
/// able to convert to a JoshutoEvent
#[derive(Debug)]
pub enum JoshutoEvent {
    /// A key press.
    Key(JoshutoKey),
    /// A mouse button press, release or wheel use at specific coordinates.
    Mouse(JoshutoMouseEvent),
    /// An event that cannot currently be evaluated.
    Unsupported(Vec<u8>),
}

/// All the keys supported by Joshuto. Different backends support different keys and have different
/// schemes for receiving modifiers. In Order to use a backend a mapping from its Key definition
/// must exist to convert all keys into `JoshutoKey`s
#[derive(Debug)]
pub enum JoshutoKey {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Backward Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    F(u8),
    /// Normal character.
    Char(char),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,

    #[doc(hidden)]
    __IsNotComplete,
}

impl ToString for JoshutoKey {
    fn to_string(&self) -> String {
        match self {
            JoshutoKey::Char(c) => format!("{}", c),
            JoshutoKey::Ctrl(c) => format!("ctrl+{}", c),
            JoshutoKey::Left => "arrow_left".to_string(),
            JoshutoKey::Right => "arrow_right".to_string(),
            JoshutoKey::Up => "arrow_up".to_string(),
            JoshutoKey::Down => "arrow_down".to_string(),
            JoshutoKey::Backspace => "backspace".to_string(),
            JoshutoKey::Home => "home".to_string(),
            JoshutoKey::End => "end".to_string(),
            JoshutoKey::PageUp => "page_up".to_string(),
            JoshutoKey::PageDown => "page_down".to_string(),
            JoshutoKey::BackTab => "backtab".to_string(),
            JoshutoKey::Insert => "insert".to_string(),
            JoshutoKey::Delete => "delete".to_string(),
            JoshutoKey::Esc => "escape".to_string(),
            JoshutoKey::F(i) => format!("f{}", i),
            k => format!("{:?}", k),
        }
    }
}

#[derive(Debug)]
pub enum JoshutoMouseEvent {
    /// A mouse button was pressed.
    ///
    /// The coordinates are one-based.
    Press(JoshutoMouseButton, u16, u16),

    /// A mouse button was released.
    ///
    /// The coordinates are one-based.
    Release(u16, u16),

    /// A mouse button is held over the given coordinates.
    ///
    /// The coordinates are one-based.
    Hold(u16, u16),
}

/// A mouse button.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JoshutoMouseButton {
    /// The left mouse button.
    Left,

    /// The right mouse button.
    Right,

    /// The middle mouse button.
    Middle,

    /// Mouse wheel is going up.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelUp,

    /// Mouse wheel is going down.
    ///
    /// This event is typically only used with Mouse::Press.
    WheelDown,
}

impl ToString for JoshutoMouseEvent {
    fn to_string(&self) -> String {
        let k = self;
        format!("{:?}", k)
    }
}

impl ToString for JoshutoEvent {
    fn to_string(&self) -> String {
        match self {
            JoshutoEvent::Key(key) => key.to_string(),
            JoshutoEvent::Mouse(mouse) => mouse.to_string(),
            JoshutoEvent::Unsupported(v) => format!("{:?}", v),
        }
    }
}

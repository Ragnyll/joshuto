use super::joshuto_event::{JoshutoEvent, JoshutoKey, JoshutoMouseButton, JoshutoMouseEvent};
use std::convert::From;
use termion::event::{
    Event as TermionEvent, Key as TermionKey, MouseButton as TermionMouseButton,
    MouseEvent as TermionMouseEvent,
};

impl From<TermionEvent> for JoshutoEvent {
    fn from(event: TermionEvent) -> Self {
        match event {
            TermionEvent::Key(key) => match key {
                TermionKey::Backspace => JoshutoEvent::Key(JoshutoKey::Backspace),
                TermionKey::Left => JoshutoEvent::Key(JoshutoKey::Left),
                TermionKey::Right => JoshutoEvent::Key(JoshutoKey::Right),
                TermionKey::Up => JoshutoEvent::Key(JoshutoKey::Up),
                TermionKey::Down => JoshutoEvent::Key(JoshutoKey::Down),
                TermionKey::Home => JoshutoEvent::Key(JoshutoKey::Home),
                TermionKey::End => JoshutoEvent::Key(JoshutoKey::End),
                TermionKey::PageUp => JoshutoEvent::Key(JoshutoKey::PageUp),
                TermionKey::PageDown => JoshutoEvent::Key(JoshutoKey::PageDown),
                TermionKey::BackTab => JoshutoEvent::Key(JoshutoKey::BackTab),
                TermionKey::Delete => JoshutoEvent::Key(JoshutoKey::Delete),
                TermionKey::Insert => JoshutoEvent::Key(JoshutoKey::Insert),
                TermionKey::F(function_key_number) => {
                    JoshutoEvent::Key(JoshutoKey::F(function_key_number))
                }
                TermionKey::Char(char) => JoshutoEvent::Key(JoshutoKey::Char(char)),
                TermionKey::Alt(char) => JoshutoEvent::Key(JoshutoKey::Alt(char)),
                TermionKey::Ctrl(char) => JoshutoEvent::Key(JoshutoKey::Ctrl(char)),
                TermionKey::Null => JoshutoEvent::Key(JoshutoKey::Null),
                TermionKey::Esc => JoshutoEvent::Key(JoshutoKey::Esc),
                // kinda sus
                _ => JoshutoEvent::Unsupported(vec![0_u8]),
            },
            TermionEvent::Mouse(mouse_event) => match mouse_event {
                TermionMouseEvent::Press(mouse_button, x, y) => JoshutoEvent::Mouse(
                    JoshutoMouseEvent::Press(JoshutoMouseButton::from(mouse_button), x, y),
                ),
                TermionMouseEvent::Release(x, y) => {
                    JoshutoEvent::Mouse(JoshutoMouseEvent::Release(x, y))
                }
                TermionMouseEvent::Hold(x, y) => JoshutoEvent::Mouse(JoshutoMouseEvent::Hold(x, y)),
            },
            TermionEvent::Unsupported(unsupported) => JoshutoEvent::Unsupported(unsupported),
        }
    }
}

impl From<TermionMouseButton> for JoshutoMouseButton {
    fn from(mouse_button: TermionMouseButton) -> Self {
        match mouse_button {
            TermionMouseButton::Left => JoshutoMouseButton::Left,
            TermionMouseButton::Middle => JoshutoMouseButton::Middle,
            TermionMouseButton::Right => JoshutoMouseButton::Right,
            TermionMouseButton::WheelUp => JoshutoMouseButton::WheelUp,
            TermionMouseButton::WheelDown => JoshutoMouseButton::WheelDown,
        }
    }
}

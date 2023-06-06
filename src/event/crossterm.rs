use super::joshuto_event::{JoshutoEvent, JoshutoKey, JoshutoMouseButton, JoshutoMouseEvent};
use std::convert::From;
use crossterm::event::{Event as CrosstermEvent, KeyEvent as CrosstermKeyEvent, KeyModifiers as CrosstermKeyModifers, KeyCode as CrosstermKeyCode};

impl From<CrosstermEvent> for JoshutoEvent {
    fn from(event: CrosstermEvent) -> Self {
        match event {
            CrosstermEvent::FocusLost => todo!(),
            CrosstermEvent::FocusGained => todo!(),
            #[allow(unused_variables)]
            CrosstermEvent::Paste(string) => todo!(),
            #[allow(unused_variables)]
            CrosstermEvent::Resize(x, y) => todo!(),
            #[allow(unused_variables)]
            CrosstermEvent::Mouse(mouse_event) => todo!(),
            CrosstermEvent::Key(key_event) => {
                let key_code = key_event.code;
                let modifiers = key_event.modifiers;
                // let key_event_kind =  key_event.kind; // need to look into this for windows

                match modifiers {
                    CrosstermKeyModifers::CONTROL => {
                        match key_code {
                            CrosstermKeyCode::Char(char) => return JoshutoEvent::Key(JoshutoKey::Ctrl(char)),
                            _ => { todo!() },
                        }
                    },
                    CrosstermKeyModifers::ALT => {
                        match key_code {
                            CrosstermKeyCode::Char(char) => return JoshutoEvent::Key(JoshutoKey::Alt(char)),
                            _ => { todo!() },
                        }
                    }
                    CrosstermKeyModifers::NONE => { /* NOOP just fall through to matching on key_codea */ }
                    _ => {
                        todo!();
                    }
                };

                match key_code {
                    CrosstermKeyCode::Esc => JoshutoEvent::Key(JoshutoKey::Esc),
                    CrosstermKeyCode::Null => JoshutoEvent::Key(JoshutoKey::Null),
                    CrosstermKeyCode::Backspace => JoshutoEvent::Key(JoshutoKey::Backspace),
                    CrosstermKeyCode::Left => JoshutoEvent::Key(JoshutoKey::Left),
                    CrosstermKeyCode::Right => JoshutoEvent::Key(JoshutoKey::Right),
                    CrosstermKeyCode::Up => JoshutoEvent::Key(JoshutoKey::Up),
                    CrosstermKeyCode::Down => JoshutoEvent::Key(JoshutoKey::Down),
                    CrosstermKeyCode::Home => JoshutoEvent::Key(JoshutoKey::Home),
                    CrosstermKeyCode::End => JoshutoEvent::Key(JoshutoKey::End),
                    CrosstermKeyCode::PageUp => JoshutoEvent::Key(JoshutoKey::PageUp),
                    CrosstermKeyCode::PageDown => JoshutoEvent::Key(JoshutoKey::PageDown),
                    CrosstermKeyCode::BackTab => JoshutoEvent::Key(JoshutoKey::BackTab),
                    CrosstermKeyCode::Delete => JoshutoEvent::Key(JoshutoKey::Delete),
                    CrosstermKeyCode::Insert => JoshutoEvent::Key(JoshutoKey::Insert),
                    CrosstermKeyCode::F(function_key_number) => {
                        JoshutoEvent::Key(JoshutoKey::F(function_key_number))
                    }
                    CrosstermKeyCode::Char(char) => JoshutoEvent::Key(JoshutoKey::Char(char)),
                    _ => JoshutoEvent::Unsupported(vec![0_u8]),
                }
            }

        }
    }
}

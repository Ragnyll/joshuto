use crossterm::event::{Event, KeyCode, KeyEvent, MouseEventKind};

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for KeyEvent {
    fn to_string(&self) -> String {
        match self.code {
            KeyCode::Char(c) => format!("{}", c),
            //KeyCode::Ctrl(c) => format!("ctrl+{}", c),
            KeyCode::Left => "arrow_left".to_string(),
            KeyCode::Right => "arrow_right".to_string(),
            KeyCode::Up => "arrow_up".to_string(),
            KeyCode::Down => "arrow_down".to_string(),
            KeyCode::Backspace => "backspace".to_string(),
            KeyCode::Home => "home".to_string(),
            KeyCode::End => "end".to_string(),
            KeyCode::PageUp => "page_up".to_string(),
            KeyCode::PageDown => "page_down".to_string(),
            KeyCode::BackTab => "backtab".to_string(),
            KeyCode::Insert => "insert".to_string(),
            KeyCode::Delete => "delete".to_string(),
            KeyCode::Esc => "escape".to_string(),
            KeyCode::F(i) => format!("f{}", i),
            k => format!("{:?}", k),
        }
    }
}

impl ToString for MouseEventKind {
    fn to_string(&self) -> String {
        let k = *self;
        format!("{:?}", k)
    }
}

impl ToString for Event {
    fn to_string(&self) -> String {
        match self {
            Event::Key(key) => key.to_string(),
            Event::Mouse(mouse) => todo!("ToString For mouse event"),
            _ => todo!("ToString for other event")
            //Event::Mouse(mouse) => mouse.to_string(),
            //Event::Unsupported(v) => format!("{:?}", v),
        }
    }
}

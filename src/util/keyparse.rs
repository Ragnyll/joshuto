use crate::event::joshuto_event::{
    JoshutoEvent, JoshutoKey, JoshutoMouseButton, JoshutoMouseEvent,
};

pub fn str_to_event(s: &str) -> Option<JoshutoEvent> {
    if let Some(k) = str_to_key(s) {
        Some(JoshutoEvent::Key(k))
    } else {
        str_to_mouse(s).map(JoshutoEvent::Mouse)
    }
}

pub fn str_to_key(s: &str) -> Option<JoshutoKey> {
    if s.is_empty() {
        return None;
    }

    let key = match s {
        "backspace" => Some(JoshutoKey::Backspace),
        "backtab" => Some(JoshutoKey::BackTab),
        "arrow_left" => Some(JoshutoKey::Left),
        "arrow_right" => Some(JoshutoKey::Right),
        "arrow_up" => Some(JoshutoKey::Up),
        "arrow_down" => Some(JoshutoKey::Down),
        "home" => Some(JoshutoKey::Home),
        "end" => Some(JoshutoKey::End),
        "page_up" => Some(JoshutoKey::PageUp),
        "page_down" => Some(JoshutoKey::PageDown),
        "delete" => Some(JoshutoKey::Delete),
        "insert" => Some(JoshutoKey::Insert),
        "escape" => Some(JoshutoKey::Esc),
        "f1" => Some(JoshutoKey::F(1)),
        "f2" => Some(JoshutoKey::F(2)),
        "f3" => Some(JoshutoKey::F(3)),
        "f4" => Some(JoshutoKey::F(4)),
        "f5" => Some(JoshutoKey::F(5)),
        "f6" => Some(JoshutoKey::F(6)),
        "f7" => Some(JoshutoKey::F(7)),
        "f8" => Some(JoshutoKey::F(8)),
        "f9" => Some(JoshutoKey::F(9)),
        "f10" => Some(JoshutoKey::F(10)),
        "f11" => Some(JoshutoKey::F(11)),
        "f12" => Some(JoshutoKey::F(12)),
        _ => None,
    };

    if key.is_some() {
        return key;
    }

    if s.starts_with("ctrl+") {
        let ch = s.chars().nth("ctrl+".len());
        let key = ch.map(JoshutoKey::Ctrl);
        return key;
    } else if s.starts_with("alt+") {
        let ch = s.chars().nth("alt+".len());
        let key = ch.map(JoshutoKey::Alt);
        return key;
    } else if s.len() == 1 {
        let ch = s.chars().next();
        let key = ch.map(JoshutoKey::Char);
        return key;
    }
    None
}

pub fn str_to_mouse(s: &str) -> Option<JoshutoMouseEvent> {
    match s {
        "scroll_up" => Some(JoshutoMouseEvent::Press(JoshutoMouseButton::WheelUp, 0, 0)),
        "scroll_down" => Some(JoshutoMouseEvent::Press(
            JoshutoMouseButton::WheelDown,
            0,
            0,
        )),
        _ => None,
    }
}

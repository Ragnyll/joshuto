use crossterm::event::{Event, KeyCode, MouseButton, MouseEvent};

pub fn str_to_event(s: &str) -> Option<KeyCode> {
    //if let Some(k) = str_to_key(s) {
        //Some(Event::Key(k))
    //} else {
        //str_to_mouse(s).map(Event::Mouse)
    //}
    str_to_key(s)
}

pub fn str_to_key(s: &str) -> Option<KeyCode> {
    if s.is_empty() {
        return None;
    }

    let key = match s {
        "backspace" => Some(KeyCode::Backspace),
        "backtab" => Some(KeyCode::BackTab),
        "arrow_left" => Some(KeyCode::Left),
        "arrow_right" => Some(KeyCode::Right),
        "arrow_up" => Some(KeyCode::Up),
        "arrow_down" => Some(KeyCode::Down),
        "home" => Some(KeyCode::Home),
        "end" => Some(KeyCode::End),
        "page_up" => Some(KeyCode::PageUp),
        "page_down" => Some(KeyCode::PageDown),
        "delete" => Some(KeyCode::Delete),
        "insert" => Some(KeyCode::Insert),
        "escape" => Some(KeyCode::Esc),
        "f1" => Some(KeyCode::F(1)),
        "f2" => Some(KeyCode::F(2)),
        "f3" => Some(KeyCode::F(3)),
        "f4" => Some(KeyCode::F(4)),
        "f5" => Some(KeyCode::F(5)),
        "f6" => Some(KeyCode::F(6)),
        "f7" => Some(KeyCode::F(7)),
        "f8" => Some(KeyCode::F(8)),
        "f9" => Some(KeyCode::F(9)),
        "f10" => Some(KeyCode::F(10)),
        "f11" => Some(KeyCode::F(11)),
        "f12" => Some(KeyCode::F(12)),
        _ => None,
    };

    if key.is_some() {
        return key;
    }

    //if s.starts_with("ctrl+") {
        //let ch = s.chars().nth("ctrl+".len());
        //let key = ch.map(KeyCode::Ctrl);
        //return key;
    //} else if s.starts_with("alt+") {
        //let ch = s.chars().nth("alt+".len());
        //let key = ch.map(KeyCode::Alt);
        //return key;
    //} else if s.len() == 1 {
        //let ch = s.chars().next();
        //let key = ch.map(KeyCode::Char);
        //return key;
    //}
    None
}

pub fn str_to_mouse(s: &str) -> Option<MouseEvent> {
    match s {
        //"scroll_up" => Some(MouseEvent::Press(MouseButton::WheelUp, 0, 0)),
        //"scroll_down" => Some(MouseEvent::Press(MouseButton::WheelDown, 0, 0)),
        _ => None,
    }
}

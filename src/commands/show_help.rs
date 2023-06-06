use std::cmp::Ordering;

use crate::event::joshuto_event::{JoshutoEvent, JoshutoKey};
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::event::process_event;
use crate::event::AppEvent;
use crate::key_command::{Command, CommandKeybind};
use crate::ui::widgets;
use crate::ui::widgets::TuiHelp;
use crate::ui::AppBackend;

pub fn help_loop<T: AppBackend>(
    context: &mut AppContext,
    backend: &mut T,
    keymap_t: &AppKeyMapping,
) -> JoshutoResult {
    context.flush_event();

    let mut offset = 0;
    let mut search_query = String::new();
    let mut sort_by = 1;

    loop {
        let keymap = if search_query.is_empty() {
            widgets::get_keymap_table(&keymap_t.default_view, &search_query, sort_by)
        } else {
            widgets::get_keymap_table(&keymap_t.default_view, &search_query[1..], sort_by)
        };

        context.remove_external_preview();
        backend.render(TuiHelp::new(&keymap, &mut offset, &search_query));

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        match event {
            AppEvent::Backend(event) => {
                if search_query.is_empty() {
                    match event {
                        JoshutoEvent::Key(JoshutoKey::Esc) => break,
                        JoshutoEvent::Key(JoshutoKey::Char('1')) => sort_by = 0,
                        JoshutoEvent::Key(JoshutoKey::Char('2')) => sort_by = 1,
                        JoshutoEvent::Key(JoshutoKey::Char('3')) => sort_by = 2,
                        JoshutoEvent::Key(JoshutoKey::Char('/')) => search_query.push('/'),
                        event => {
                            if let Some(CommandKeybind::SimpleKeybind(command)) =
                                keymap_t.help_view.get(&event)
                            {
                                match command {
                                    Command::CursorMoveUp { .. } => move_offset(&mut offset, -1),
                                    Command::CursorMoveDown { .. } => move_offset(&mut offset, 1),
                                    Command::CursorMoveHome => offset = 0,
                                    Command::CursorMoveEnd => offset = 255,
                                    Command::CursorMovePageUp(_) => move_offset(&mut offset, -10),
                                    Command::CursorMovePageDown(_) => move_offset(&mut offset, 10),
                                    Command::CloseTab | Command::Help => break,
                                    _ => (),
                                }
                            }
                        }
                    }
                } else {
                    match event {
                        JoshutoEvent::Key(JoshutoKey::Esc) => search_query.clear(),
                        JoshutoEvent::Key(JoshutoKey::Backspace) => {
                            search_query.pop();
                        }
                        JoshutoEvent::Key(JoshutoKey::Char(chr)) => search_query.push(chr),
                        _ => (),
                    }
                }
                context.flush_event();
            }
            _ => process_event::process_noninteractive(event, context),
        }
    }

    Ok(())
}

// offset is a u8, so if we make it negative program will fail.
// This function prevents this error
fn move_offset(offset: &mut u8, moving_amount: i8) {
    match moving_amount.cmp(&0) {
        Ordering::Greater => {
            *offset += moving_amount as u8;
        }
        Ordering::Less => {
            if *offset > -moving_amount as u8 {
                *offset -= -moving_amount as u8;
            } else {
                *offset = 0;
            }
        }
        Ordering::Equal => (),
    }
}

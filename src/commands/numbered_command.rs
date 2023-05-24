use crossterm::event::{Event, KeyCode};

use crate::commands::cursor_move;
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::event::process_event;
use crate::event::AppEvent;
use crate::key_command::{CommandKeybind, NumberedExecute};
use crate::ui::views::TuiView;
use crate::ui::AppBackend;

pub fn numbered_command<T: AppBackend>(
    context: &mut AppContext,
    backend: &mut T,
    keymap: &AppKeyMapping,
    first_char: char,
) -> JoshutoResult {
    context.flush_event();
    let mut prefix = String::from(first_char);

    loop {
        context.message_queue_mut().push_info(prefix.clone());
        backend.render(TuiView::new(context));
        context.message_queue_mut().pop_front();

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        let num_prefix = match prefix.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                context.message_queue_mut().pop_front();
                return Err(JoshutoError::new(
                    JoshutoErrorKind::ParseError,
                    "Number is too big".to_string(),
                ));
            }
        };

        match event {
            AppEvent::Crossterm(event) => {
                if let Event::Key(keyevent) = event {
                    match keyevent.code {
                        KeyCode::Esc => return Ok(()),
                        KeyCode::Char('g') => {
                            cursor_move::cursor_move(context, num_prefix - 1);
                            return Ok(());
                        }
                        KeyCode::Char(c) if c.is_numeric() => {
                            prefix.push(c);
                        }
                        _ => {
                            todo!("numbered_command aint all there");
                        }
                        // TODO: i'm not exactly sure what this does. com back to it.
                        //key => match keymap.default_view.get(&key) {
                            //Some(CommandKeybind::SimpleKeybind(command)) => {
                                //return command
                                    //.numbered_execute(num_prefix, context, backend, keymap);
                            //}
                            //_ => {
                                //return Err(JoshutoError::new(
                                    //JoshutoErrorKind::UnrecognizedCommand,
                                    //"Command cannot be prefixed by a number or does not exist"
                                        //.to_string(),
                                //));
                            //}
                        //},
                    }
                } else {
                    panic!("Im not sure how we got here.")
                }
                context.flush_event();
            }
            event => process_event::process_noninteractive(event, context),
        }
    }
}

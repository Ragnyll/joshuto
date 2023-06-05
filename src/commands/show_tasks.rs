use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::event::process_event;
use crate::event::AppEvent;
use crate::key_command::{Command, CommandKeybind};
use crate::ui::views::TuiWorkerView;
use crate::ui::AppBackend;

pub fn show_tasks<T: AppBackend>(
    context: &mut AppContext,
    backend: &mut T,
    keymap_t: &AppKeyMapping,
) -> JoshutoResult {
    context.flush_event();

    loop {
        backend.render(TuiWorkerView::new(context));

        if let Ok(event) = context.poll_event() {
            match event {
                AppEvent::Backend(key) => {
                    let key = key;
                    match keymap_t.task_view.get(&key) {
                        None => {
                            context
                                .message_queue_mut()
                                .push_info(format!("Unmapped input: {}", key.to_string()));
                        }
                        Some(CommandKeybind::SimpleKeybind(command)) => {
                            if let Command::ShowTasks = command {
                                break;
                            }
                        }
                        Some(CommandKeybind::CompositeKeybind(m)) => {
                            let cmd =
                                process_event::poll_event_until_simple_keybind(backend, context, m);

                            if let Some(Command::ShowTasks) = cmd {
                                break;
                            }
                        }
                    }
                    context.flush_event();
                }
                event => process_event::process_noninteractive(event, context),
            };
        }
    }
    Ok(())
}

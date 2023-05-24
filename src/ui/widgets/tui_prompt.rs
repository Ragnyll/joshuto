use crossterm::event::{Event, KeyCode};
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Clear, Paragraph, Wrap};

use crate::context::AppContext;
use crate::event::process_event;
use crate::event::AppEvent;
use crate::ui::backend::traits::AppBackend;
use crate::ui::views::TuiView;

pub struct TuiPrompt<'a> {
    prompt: &'a str,
}

impl<'a> TuiPrompt<'a> {
    pub fn new(prompt: &'a str) -> Self {
        Self { prompt }
    }

    pub fn get_key<T: AppBackend>(&mut self, backend: &mut T, context: &mut AppContext) -> KeyCode {
        let terminal = backend.terminal_mut();

        context.flush_event();
        loop {
            let _ = terminal.draw(|frame| {
                let f_size: Rect = frame.size();
                if f_size.height == 0 {
                    return;
                }

                {
                    let mut view = TuiView::new(context);
                    view.show_bottom_status = false;
                    frame.render_widget(view, f_size);
                }

                let prompt_style = Style::default().fg(Color::LightYellow);

                let text = Span::styled(self.prompt, prompt_style);

                let textfield_rect = Rect {
                    x: 0,
                    y: f_size.height - 1,
                    width: f_size.width,
                    height: 1,
                };

                frame.render_widget(Clear, textfield_rect);
                frame.render_widget(
                    Paragraph::new(text).wrap(Wrap { trim: true }),
                    textfield_rect,
                );
            });

            if let Ok(event) = context.poll_event() {
                match event {
                    AppEvent::Crossterm(Event::Key(key)) => {
                        return key.code;
                    }
                    AppEvent::Crossterm(_) => {
                        context.flush_event();
                    }
                    event => process_event::process_noninteractive(event, context),
                };
            }
        }
    }
}

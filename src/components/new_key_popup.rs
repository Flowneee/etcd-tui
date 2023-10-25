use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    style::{Style, Stylize},
    widgets::{Borders, Clear},
};
use tui_textarea::{Input, Key, TextArea};

use crate::{
    events::KeyEventState,
    shared_state::SharedState,
    ui::{calculate_center_rect, titled_block, Frame},
};

use super::Component;

// TODO: validate that key doesn't exist

#[derive(Clone, Debug)]
pub enum NewKeyResult {
    Cancel,
    Done(String),
}

impl NewKeyResult {
    pub fn as_done(self) -> Option<String> {
        if let Self::Done(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

pub struct NewKeyPopup {
    textarea: TextArea<'static>,
    result: Option<NewKeyResult>,

    is_visible: bool,

    shared_state: SharedState,
}

impl NewKeyPopup {
    pub fn new(shared_state: SharedState) -> Self {
        let mut textarea = TextArea::default();
        textarea.set_cursor_line_style(Style::default());
        textarea.set_placeholder_text("Enter new key name");

        Self {
            textarea,
            result: None,

            is_visible: false,

            shared_state,
        }
    }

    fn set_done(&mut self) {
        if let Some(x) = self.textarea.lines().get(0).cloned() {
            if !x.is_empty() {
                self.result = Some(NewKeyResult::Done(x))
            }
        }
    }

    pub fn status(&self) -> Option<NewKeyResult> {
        self.result.clone()
    }
}

impl Component for NewKeyPopup {
    fn handle_key_event(&mut self, event: KeyEvent) -> Result<KeyEventState> {
        if self.is_visible() {
            match event.into() {
                Input { key: Key::Esc, .. } => {
                    self.result = Some(NewKeyResult::Cancel);
                }
                Input {
                    key: Key::Enter, ..
                } => {
                    self.set_done();
                }
                input => {
                    self.textarea.input(input);
                }
            }
            Ok(KeyEventState::Consumed)
        } else {
            Ok(KeyEventState::NotConsumed)
        }
    }

    fn draw(&mut self, frame: &mut Frame, _: Rect) {
        let block = titled_block("New key").borders(Borders::ALL).on_dark_gray();
        self.textarea.set_block(block);

        let rect = calculate_center_rect(30, 3, frame.size());

        frame.render_widget(Clear, rect);
        frame.render_widget(self.textarea.widget(), rect);
    }

    fn set_visibility(&mut self, value: bool) {
        self.is_visible = value;
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn context_help(&self) -> Vec<String> {
        if self.is_visible() {
            vec!["(Enter) done".into(), "(Esc) cancel".into()]
        } else {
            vec![]
        }
    }
}

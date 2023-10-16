


use crossterm::{
    event::{Event},
};
use ratatui::{
    prelude::{Rect},
};
use tui_textarea::{Input, Key, TextArea};

use super::{main_titled_block, Frame};

pub(super) struct ValueEditorScene {
    is_in_editing_mode: bool,
    editor_textarea: TextArea<'static>,
    key: String,
    original_key_value: String,
}

impl ValueEditorScene {
    pub(super) fn new(key: String, value: String) -> Self {
        Self {
            is_in_editing_mode: false,
            editor_textarea: TextArea::from(value.split('\n')),
            key,
            original_key_value: value,
        }
    }

    pub(super) fn draw(&mut self, frame: &mut Frame, layout_rect: Rect) {
        let mode = if self.is_in_editing_mode {
            "editing"
        } else {
            "not editing"
        };
        let value_editor_title = format!("Key '{}' editor ({mode})", self.key);
        self.editor_textarea
            .set_block(main_titled_block(value_editor_title));

        frame.render_widget(self.editor_textarea.widget(), layout_rect)
    }

    /// Handle event, returns whether scene is finished or not.
    pub(super) fn handle_event(&mut self, event: Event) -> bool {
        if self.is_in_editing_mode {
            match event.into() {
                Input { key: Key::Esc, .. } => {
                    self.is_in_editing_mode = false;
                }
                rest => {
                    self.editor_textarea.input(rest);
                }
            }
        } else {
            match event.into() {
                Input {
                    key: Key::Char('e'),
                    ..
                } => {
                    self.is_in_editing_mode = true;
                }
                Input { key: Key::Esc, .. } => return true,
                _ => {}
            }
        }
        false
    }

    pub(super) fn help(&self) -> String {
        if self.is_in_editing_mode {
            "(Esc) exit editing mode".into()
        } else {
            "(e) enter editing mode, (Esc) return to key selection".into()
        }
    }
}

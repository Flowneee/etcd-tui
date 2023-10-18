use anyhow::Result;

use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use tui_textarea::{Input, Key, TextArea};

use crate::{
    events::{Event, KeyEventState},
    ui::{main_titled_block, Frame},
    SharedState,
};

use super::{confirmation_popup::ConfirmationResult, Component, ConfirmationPopup, ForegroundTask};

pub struct ValueEditor {
    shared_state: SharedState,

    is_visible: bool,

    is_in_editing_mode: bool,
    editor_textarea: TextArea<'static>,
    key: String,
    original_key_value: String,

    confirmation_popup: Option<ConfirmationPopup>,
    put_key_task: ForegroundTask<Result<()>>,
}

impl ValueEditor {
    pub fn new(shared_state: SharedState) -> Self {
        Self {
            shared_state: shared_state.clone(),

            is_visible: false,

            is_in_editing_mode: false,
            editor_textarea: TextArea::new(vec![]),
            key: String::new(),
            original_key_value: String::new(),

            confirmation_popup: None,
            put_key_task: ForegroundTask::new("Saving key", shared_state),
        }
    }

    pub fn editor_content(&self) -> String {
        self.editor_textarea.lines().join("\n")
    }

    pub fn open_key(&mut self, key: String, value: String) {
        let sanitized_value = value.replace("\r\n", "\n").replace('\r', "\n");
        self.key = key;
        self.editor_textarea = TextArea::from(sanitized_value.split('\n'));
        self.original_key_value = sanitized_value;

        self.show();
    }

    fn put_key(&mut self) {
        let key = self.key.clone();
        let value = self.editor_content();
        self.put_key_task
            .start(|s| async move { s.put_key(&key, value).await });
    }

    fn value_has_changed(&self) -> bool {
        self.editor_content() != self.original_key_value
    }

    fn title_status(&self) -> String {
        let mode = if self.is_in_editing_mode {
            "editing"
        } else {
            "not editing"
        };
        let changed = if self.value_has_changed() {
            "changed"
        } else {
            "not changed"
        };
        format!("{mode}, {changed}")
    }

    fn edit_done(&self) -> Result<()> {
        self.shared_state.send_event(Event::KeyEditDone)
    }

    fn prompt_save_confirmation(&mut self) {
        let mut popup = ConfirmationPopup::new("Save key?", self.shared_state.clone());
        popup.show();
        self.confirmation_popup = Some(popup);
    }
}

impl Component for ValueEditor {
    fn handle_key_event(&mut self, event: KeyEvent) -> Result<KeyEventState> {
        if self.is_visible() {
            key_event!(self.put_key_task.handle_key_event(event));
            if let Some(ref mut x) = self.confirmation_popup {
                key_event!(x.handle_key_event(event));
            }

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
                        key: Key::Enter | Key::Char('e'),
                        ..
                    } => {
                        self.is_in_editing_mode = true;
                    }
                    Input { key: Key::Esc, .. } => {
                        if self.value_has_changed() {
                            self.prompt_save_confirmation();
                        } else {
                            self.edit_done()?;
                        }
                    }
                    _ => {}
                }
            }
            Ok(KeyEventState::Consumed)
        } else {
            Ok(KeyEventState::NotConsumed)
        }
    }

    fn update(&mut self) -> Result<()> {
        if let Some(ref mut x) = self.confirmation_popup {
            if let Some(result) = x.status() {
                match result {
                    ConfirmationResult::Yes => {
                        self.put_key();
                    }
                    ConfirmationResult::No => {
                        self.edit_done()?;
                    }
                    ConfirmationResult::Cancel => {}
                }
                self.confirmation_popup = None;
            }
        }

        if let Some(result) = self.put_key_task.try_ready() {
            result?;
            self.edit_done()?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        if self.is_visible() {
            let value_editor_title = format!("Key '{}' editor ({})", self.key, self.title_status());
            self.editor_textarea
                .set_block(main_titled_block(value_editor_title));

            frame.render_widget(self.editor_textarea.widget(), rect);

            self.put_key_task.draw(frame, rect);
            if let Some(ref mut x) = self.confirmation_popup {
                x.draw(frame, rect);
            }
        }
    }

    fn set_visibility(&mut self, value: bool) {
        self.is_visible = value;
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn context_help(&self) -> Vec<String> {
        if self.is_visible() {
            if let Some(ref x) = self.confirmation_popup {
                return x.context_help();
            }

            if self.is_in_editing_mode {
                vec!["(Esc) exit editing mode".into()]
            } else {
                vec![
                    "(e/Enter) enter editing mode".into(),
                    "(Esc) return to key selection".into(),
                ]
            }
        } else {
            vec![]
        }
    }
}

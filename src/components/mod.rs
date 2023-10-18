pub use self::{
    confirmation_popup::ConfirmationPopup, context_help::ContextHelp,
    foreground_task::ForegroundTask, key_selector::KeySelector, value_editor::ValueEditor,
};

use anyhow::Result;

use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;

use crate::{events::KeyEventState, ui::Frame};

mod confirmation_popup;
mod context_help;
mod foreground_task;
mod key_selector;
mod value_editor;

pub trait Component {
    fn handle_key_event(&mut self, event: KeyEvent) -> Result<KeyEventState> {
        Ok(KeyEventState::NotConsumed)
    }

    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {}

    fn set_visibility(&mut self, value: bool) {}

    fn is_visible(&self) -> bool {
        false
    }

    fn context_help(&self) -> Vec<String> {
        vec![]
    }

    fn show(&mut self) {
        self.set_visibility(true);
    }

    fn hide(&mut self) {
        self.set_visibility(false);
    }
}

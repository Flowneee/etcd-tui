pub use self::{context_help::ContextHelp, key_selector::KeySelector, value_editor::ValueEditor};

use anyhow::Result;

use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;

use crate::ui::Frame;

mod context_help;
mod key_selector;
mod value_editor;

pub trait Component {
    fn handle_key_event(&mut self, event: KeyEvent) -> Result<()>;

    fn update(&mut self) -> Result<()>;

    fn draw(&mut self, frame: &mut Frame, rect: Rect);

    fn set_visibility(&mut self, value: bool);

    fn is_visible(&self) -> bool;

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

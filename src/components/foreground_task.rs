use std::ops::{Deref, DerefMut};

use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Alignment, Rect},
    style::Stylize,
    widgets::{Block, Borders, Clear, Paragraph},
};
use tui_textarea::{Input, Key};

use super::Component;
use crate::{
    events::KeyEventState,
    shared_state::SharedState,
    ui::{calculate_center_rect, Frame},
    utils::AsyncTask,
};

/// Wrapper around `AsyncTask`, which draw popup, if there is an active task, inhibiting
/// any input and allowing to cancel task.
pub struct ForegroundTask<T> {
    description: String,

    task: AsyncTask<T>,
}

impl<T> ForegroundTask<T>
where
    T: Send + 'static,
{
    pub fn new(description: impl ToString, shared_state: SharedState) -> Self {
        Self {
            description: description.to_string(),
            task: AsyncTask::new(shared_state),
        }
    }
}

impl<T> Deref for ForegroundTask<T> {
    type Target = AsyncTask<T>;

    fn deref(&self) -> &Self::Target {
        &self.task
    }
}

impl<T> DerefMut for ForegroundTask<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.task
    }
}

impl<T> Component for ForegroundTask<T>
where
    T: Send + 'static,
{
    fn handle_key_event(&mut self, event: KeyEvent) -> Result<KeyEventState> {
        if self.is_visible() {
            if matches!(event.into(), Input { key: Key::Esc, .. }) {
                self.task.abort();
            }
            Ok(KeyEventState::Consumed)
        } else {
            Ok(KeyEventState::NotConsumed)
        }
    }

    fn draw(&mut self, frame: &mut Frame, _: Rect) {
        if self.is_visible() {
            let paragraph = Paragraph::new(self.description.clone())
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).on_dark_gray());
            // TODO: calculate size of widget from description
            let rect = calculate_center_rect(30, 5, frame.size());
            frame.render_widget(Clear, rect);
            frame.render_widget(paragraph, rect);
        }
    }

    fn context_help(&self) -> Vec<String> {
        if self.is_visible() {
            vec!["(Esc) cancel".into()]
        } else {
            vec![]
        }
    }

    // NOTE: can be turned off, but not on
    fn set_visibility(&mut self, value: bool) {
        if !value {
            self.task.abort();
        }
    }

    fn is_visible(&self) -> bool {
        self.task.is_active()
    }
}

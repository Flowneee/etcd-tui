use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Clear, Padding, Paragraph},
};
use tui_textarea::{Input, Key};

use crate::{
    events::KeyEventState,
    shared_state::SharedState,
    ui::{calculate_center_rect, titled_block, Frame},
};

use super::Component;

#[derive(Copy, Clone, Debug)]
pub enum ConfirmationResult {
    Yes,
    No,
    Cancel,
}

pub struct ConfirmationPopup {
    description: String,
    result: Option<ConfirmationResult>,

    is_visible: bool,

    shared_state: SharedState,
}

impl ConfirmationPopup {
    pub fn new(description: impl ToString, shared_state: SharedState) -> Self {
        Self {
            description: description.to_string(),
            result: None,

            is_visible: false,

            shared_state,
        }
    }

    fn set_result(&mut self, result: ConfirmationResult) -> Result<()> {
        self.result = Some(result);
        self.shared_state.tick()?;
        Ok(())
    }

    pub fn status(&self) -> Option<ConfirmationResult> {
        self.result
    }
}

impl Component for ConfirmationPopup {
    fn handle_key_event(&mut self, event: KeyEvent) -> Result<KeyEventState> {
        if self.is_visible() {
            match event.into() {
                Input {
                    key: Key::Char('y'),
                    ..
                } => {
                    self.set_result(ConfirmationResult::Yes)?;
                }
                Input {
                    key: Key::Char('n'),
                    ..
                } => {
                    self.set_result(ConfirmationResult::No)?;
                }
                Input { key: Key::Esc, .. } => {
                    self.set_result(ConfirmationResult::Cancel)?;
                }
                _ => {}
            }
            Ok(KeyEventState::Consumed)
        } else {
            Ok(KeyEventState::NotConsumed)
        }
    }

    fn draw(&mut self, frame: &mut Frame, _: Rect) {
        let block = titled_block("Confirmation")
            .borders(Borders::ALL)
            .on_dark_gray();

        // TODO: calculate size of widget from description
        let rect = calculate_center_rect(30, 5, frame.size());

        let inner_layout = Layout::default()
            .constraints(vec![
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Max(1),
                Constraint::Min(0),
            ])
            .direction(Direction::Vertical)
            .split(block.inner(rect));

        let description_paragraph =
            Paragraph::new(self.description.clone()).alignment(Alignment::Center);
        let y_n_esc_paragraph = Paragraph::new("y/n/Esc").alignment(Alignment::Center);

        frame.render_widget(Clear, rect);
        frame.render_widget(block, rect);
        frame.render_widget(description_paragraph, inner_layout[0]);
        frame.render_widget(y_n_esc_paragraph, inner_layout[2]);
    }

    fn set_visibility(&mut self, value: bool) {
        self.is_visible = value;
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn context_help(&self) -> Vec<String> {
        if self.is_visible() {
            vec!["(y) yes".into(), "(n) no".into(), "(Esc) cancel".into()]
        } else {
            vec![]
        }
    }
}

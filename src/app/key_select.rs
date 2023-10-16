use std::cmp::{max, min};

use crossterm::event::KeyEvent;
use ratatui::{
    prelude::Rect,
    style::{Modifier, Style},
    widgets::{List, ListItem, ListState},
};
use tui_textarea::{Input, Key};

use crate::{
    event::Event,
    ui::{main_titled_block, Frame},
    SharedState,
};

pub(super) struct KeySelect {
    keys: Vec<String>,
    list_state: ListState,
}

impl KeySelect {
    pub(super) fn new(keys: Vec<String>) -> Self {
        Self {
            keys,
            list_state: ListState::default(),
        }
    }

    pub(super) fn draw(&mut self, frame: &mut Frame, layout_rect: Rect) {
        let items = self
            .keys
            .iter()
            .map(|x| ListItem::new(x.clone()))
            .collect::<Vec<_>>();

        let widget = List::new(items)
            .block(main_titled_block("Keys"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        frame.render_stateful_widget(widget, layout_rect, &mut self.list_state);
    }

    /// Returns name of the key, if one was selected.
    pub(super) fn update(&mut self, event: KeyEvent, shared_state: &SharedState) -> Option<String> {
        let selected_key_id = self.list_state.selected();
        match event.into() {
            Input { key: Key::Down, .. } => {
                self.list_state.select(Some(
                    selected_key_id.map_or(0, |x| min(x.saturating_add(1), self.keys.len() - 1)),
                ));
            }
            Input { key: Key::Up, .. } => {
                self.list_state.select(Some(
                    selected_key_id.map_or(0, |x| max(x.saturating_sub(1), 0)),
                ));
            }
            Input {
                key: Key::Enter, ..
            } => {
                return self
                    .list_state
                    .selected()
                    .and_then(|x| self.keys.get(x))
                    .cloned()
            }
            Input { key: Key::Esc, .. } => drop(shared_state.send_event(Event::Quit(Ok(())))),
            _ => {}
        }
        None
    }

    pub(super) fn help(&self) -> String {
        "(Up/Down) scroll list, (Enter) select key, (Esc) exit".into()
    }
}

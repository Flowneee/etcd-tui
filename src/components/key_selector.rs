use std::cmp::{max, min};

use anyhow::Result;

use crossterm::event::KeyEvent;
use ratatui::{
    prelude::Rect,
    style::{Modifier, Style},
    widgets::{List, ListItem, ListState},
};

use tui_textarea::{Input, Key};

use crate::{
    events::{Event, KeyEventState},
    ui::{main_titled_block, Frame},
    SharedState,
};

use super::{Component, ForegroundTask};

pub struct KeySelector {
    shared_state: SharedState,

    is_visible: bool,

    keys: Vec<String>,
    list_state: ListState,

    get_key_task: ForegroundTask<Result<(String, String)>>,
    load_key_list_task: ForegroundTask<Result<Vec<String>>>,
}

impl KeySelector {
    pub fn new(shared_state: SharedState) -> Self {
        Self {
            shared_state: shared_state.clone(),

            is_visible: false,

            keys: vec![],
            list_state: ListState::default(),

            get_key_task: ForegroundTask::new("Loading key", shared_state.clone()),
            load_key_list_task: ForegroundTask::new("Loading key list", shared_state),
        }
    }

    fn get_selected_key(&mut self) {
        if let Some(key) = self
            .list_state
            .selected()
            .and_then(|x| self.keys.get(x))
            .cloned()
        {
            self.get_key_task.start(|s| async move {
                let value = s.get_key(&key).await?;
                Ok((key, value))
            });
        }
    }

    fn load_keys(&mut self) {
        self.load_key_list_task
            .start(|s| async move { s.load_keys().await });
    }
}

impl Component for KeySelector {
    fn handle_key_event(&mut self, event: KeyEvent) -> Result<KeyEventState> {
        if self.is_visible() {
            key_event!(self.get_key_task.handle_key_event(event));
            key_event!(self.load_key_list_task.handle_key_event(event));

            let selected_key_id = self.list_state.selected();
            match event.into() {
                Input { key: Key::Down, .. } => {
                    self.list_state.select(Some(
                        selected_key_id
                            .map_or(0, |x| min(x.saturating_add(1), self.keys.len() - 1)),
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
                    self.get_selected_key();
                }
                Input { key: Key::Esc, .. } => {
                    self.shared_state.send_event(Event::Quit(Ok(())))?;
                }
                _ => {}
            }
            Ok(KeyEventState::Consumed)
        } else {
            Ok(KeyEventState::NotConsumed)
        }
    }

    fn update(&mut self) -> Result<()> {
        if let Some(result) = self.load_key_list_task.try_ready() {
            self.keys = result?;
        }

        if let Some(result) = self.get_key_task.try_ready() {
            let (key, value) = result?;
            let event = Event::KeyLoaded { key, value };
            self.shared_state.send_event(event)?;
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        if self.is_visible() {
            let items = self
                .keys
                .iter()
                .map(|x| ListItem::new(x.clone()))
                .collect::<Vec<_>>();

            let widget = List::new(items)
                .block(main_titled_block("Keys"))
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

            frame.render_stateful_widget(widget, rect, &mut self.list_state);

            self.get_key_task.draw(frame, rect);
            self.load_key_list_task.draw(frame, rect);
        }
    }

    fn context_help(&self) -> Vec<String> {
        if self.is_visible() {
            if self.get_key_task.is_visible() {
                return self.get_key_task.context_help();
            }

            if self.load_key_list_task.is_visible() {
                return self.load_key_list_task.context_help();
            }

            vec![
                "(Up/Down) scroll list".into(),
                "(Enter) select key".into(),
                "(Esc) exit".into(),
            ]
        } else {
            vec![]
        }
    }

    fn set_visibility(&mut self, value: bool) {
        self.is_visible = value;
    }

    fn is_visible(&self) -> bool {
        self.is_visible
    }

    fn show(&mut self) {
        self.is_visible = true;
        if !self.load_key_list_task.is_active() {
            self.load_keys();
        }
    }
}

use anyhow::Result;

use crossterm::event::KeyEvent;
use ratatui::prelude::{Constraint, Direction, Layout, Rect};

use crate::{
    components::{Component, ContextHelp, KeySelector, ValueEditor},
    events::Event,
    ui::Frame,
    SharedState,
};

pub struct App {
    key_selector: KeySelector,
    value_editor: ValueEditor,
    context_help: ContextHelp,

    shared_state: SharedState,
    app_result: Option<Result<()>>,
}

impl App {
    pub fn new(shared_state: SharedState) -> Self {
        let mut this = Self {
            key_selector: KeySelector::new(shared_state.clone()),
            value_editor: ValueEditor::new(shared_state.clone()),
            context_help: ContextHelp::new(),

            shared_state,
            app_result: None,
        };

        this.key_selector.show();

        this
    }

    pub fn should_quit(&self) -> bool {
        self.app_result.is_some()
    }

    pub fn take_result(self) -> Result<()> {
        self.app_result.expect("App result set")
    }

    pub fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Keyboard(kb) => self.handle_key_event(kb)?,
            Event::Quit(x) => {
                self.app_result = Some(x);
            }
            Event::KeyEditDone => {
                self.key_selector.show();
                self.value_editor.hide();
            }
            Event::KeyLoaded { key, value } => {
                self.key_selector.hide();
                self.value_editor.open_key(key, value);
            }
            Event::Tui(_) | Event::Tick => {
                self.update()?;
            }
        }
        Ok(())
    }

    pub fn quit(&mut self, result: Result<()>) {
        self.app_result = Some(result);
    }
}

impl Component for App {
    fn handle_key_event(&mut self, event: KeyEvent) -> Result<()> {
        self.key_selector.handle_key_event(event)?;
        self.value_editor.handle_key_event(event)?;
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.key_selector.update()?;
        self.value_editor.update()?;
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame, _: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(0), Constraint::Max(2)])
            .split(frame.size());

        self.context_help.set_help(self.context_help());
        self.context_help.draw(frame, layout[1]);

        let main_widget_layout_rect = layout[0];
        self.key_selector.draw(frame, main_widget_layout_rect);
        self.value_editor.draw(frame, main_widget_layout_rect);
    }

    fn set_visibility(&mut self, _: bool) {}

    fn is_visible(&self) -> bool {
        true
    }

    fn context_help(&self) -> Vec<String> {
        let mut helps = vec![];

        helps.extend(self.key_selector.context_help());
        helps.extend(self.value_editor.context_help());

        helps
    }
}

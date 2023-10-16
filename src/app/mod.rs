mod key_select;
mod value_edit;

use anyhow::Result;
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    widgets::{Padding, Paragraph},
};

use crate::{
    event::Event,
    ui::{main_titled_block, Frame},
    SharedState,
};

use self::{key_select::KeySelect, value_edit::ValueEdit};

pub struct App {
    app_result: Option<Result<()>>,
    scene: Scene,
}

impl App {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            app_result: None,
            scene: Scene::new(keys),
        }
    }

    pub async fn update(&mut self, event: Event, shared_state: &SharedState) {
        match event {
            Event::Tick => {}
            Event::Key(_) => self.scene.update(event, shared_state).await,
            Event::RestTerm(_) => {}
            Event::Quit(x) => {
                self.app_result = Some(x);
            }
        }
    }

    pub fn should_quit(&self) -> bool {
        self.app_result.is_some()
    }

    pub fn take_result(self) -> Result<()> {
        self.app_result.expect("App result set")
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(0), Constraint::Max(2)])
            .split(frame.size());

        frame.render_widget(self.help_block(), layout[1]);

        let main_widget_layout_rect = layout[0];
        match self.scene {
            Scene::KeySelect(ref mut scene) => scene.draw(frame, main_widget_layout_rect),
            Scene::ValueEdit(ref mut scene) => scene.draw(frame, main_widget_layout_rect),
        }
    }

    fn help_block(&self) -> Paragraph<'static> {
        Paragraph::new(self.scene.help())
            .block(main_titled_block("Hints").padding(Padding::new(1, 1, 0, 0)))
    }
}

enum Scene {
    KeySelect(KeySelect),
    ValueEdit(ValueEdit),
}

impl Scene {
    pub fn new(keys: Vec<String>) -> Self {
        Self::KeySelect(KeySelect::new(keys))
    }

    async fn update(&mut self, event: Event, shared_state: &SharedState) {
        match self {
            Scene::KeySelect(scene) => {
                if let Event::Key(x) = event {
                    if let Some(selected_key) = scene.update(x, shared_state) {
                        let value = shared_state.get_key(&selected_key).await.unwrap();

                        *self = Scene::ValueEdit(ValueEdit::new(selected_key, value));
                    }
                }
            }
            Scene::ValueEdit(scene) => {
                if let Event::Key(x) = event {
                    if scene.update(x) {
                        shared_state
                            .put_key(scene.key(), scene.editor_content())
                            .await
                            .unwrap();

                        *self = Scene::KeySelect(KeySelect::new(
                            shared_state.load_keys().await.unwrap(),
                        ));
                    }
                }
            }
        }
    }

    /// Render help message.
    fn help(&self) -> String {
        match self {
            Scene::KeySelect(scene) => scene.help(),
            Scene::ValueEdit(scene) => scene.help(),
        }
    }
}

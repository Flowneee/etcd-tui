

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
};
use ratatui::{
    prelude::{Alignment, Constraint, CrosstermBackend, Direction, Layout},
    style::{Style, Stylize},
    widgets::{block::Title, Block, Borders, Padding, Paragraph},
};



use self::{key_selection_scene::KeySelectionScene, value_editor_scene::ValueEditorScene};

mod key_selection_scene;
mod value_editor_scene;

type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;

fn main_titled_block<'a, T>(title: T) -> Block<'a>
where
    T: Into<Title<'a>>,
{
    Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .title_style(Style::default().bold())
}

fn check_key_event_pressed(key_event: &KeyEvent, code: KeyCode, alt_required: bool) -> bool {
    key_event.kind == KeyEventKind::Press
        && key_event.code == code
        && (key_event.modifiers.contains(KeyModifiers::ALT) || !alt_required)
}

enum UiState {
    KeySelection,
    ValueEdit,
}

impl UiState {
    fn is_key_selection(&self) -> bool {
        matches!(self, Self::KeySelection)
    }

    fn is_value_edit(&self) -> bool {
        matches!(self, Self::ValueEdit)
    }
}

pub(super) struct Ui {
    key_selection_scene: KeySelectionScene,
    value_editor_scene: Option<ValueEditorScene>,
    ui_state: UiState,
    should_quit: bool,
}

impl Ui {
    pub(super) fn new(keys: Vec<String>) -> Self {
        Self {
            key_selection_scene: KeySelectionScene::new(keys),
            value_editor_scene: None,
            ui_state: UiState::KeySelection,
            should_quit: false,
        }
    }

    fn help_block(&self) -> Paragraph<'static> {
        let help_text = match self.ui_state {
            UiState::KeySelection => self.key_selection_scene.help(),
            UiState::ValueEdit => self
                .value_editor_scene
                .as_ref()
                .expect("Value editor scene present")
                .help(),
        };
        Paragraph::new(help_text)
            .block(main_titled_block("Hints").padding(Padding::new(1, 1, 0, 0)))
    }

    pub(super) fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(0), Constraint::Max(2)])
            .split(frame.size());

        frame.render_widget(self.help_block(), layout[1]);

        let main_widget_layout_rect = layout[0];
        match self.ui_state {
            UiState::KeySelection => self
                .key_selection_scene
                .draw(frame, main_widget_layout_rect),
            UiState::ValueEdit => {
                self.value_editor_scene
                    .as_mut()
                    .expect("Value editor scene present")
                    .draw(frame, main_widget_layout_rect);
            }
        }
    }

    pub(super) fn handle_next_event(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(x)
                    if check_key_event_pressed(&x, KeyCode::Esc, false)
                        && self.ui_state.is_key_selection() =>
                {
                    self.should_quit = true;
                }
                rest => match self.ui_state {
                    UiState::KeySelection => {
                        if let Some(selected_key) = self.key_selection_scene.handle_event(rest) {
                            self.ui_state = UiState::ValueEdit;
                            self.value_editor_scene = Some(ValueEditorScene::new(
                                selected_key.clone(),
                                format!("Initial value for key {selected_key}"),
                            ));
                        }
                    }
                    UiState::ValueEdit => {
                        if self
                            .value_editor_scene
                            .as_mut()
                            .expect("Value editor scene present")
                            .handle_event(rest)
                        {
                            self.ui_state = UiState::KeySelection;
                        }
                    }
                },
            }
        }
        Ok(())
    }

    pub(super) fn should_quit(&self) -> bool {
        self.should_quit
    }
}

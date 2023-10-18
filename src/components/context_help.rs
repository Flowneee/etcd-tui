use ratatui::{
    prelude::Rect,
    widgets::{Padding, Paragraph},
};

use crate::{
    components::Component,
    ui::{main_titled_block, Frame},
};

pub struct ContextHelp {
    help: Vec<String>,
}

impl ContextHelp {
    pub fn new() -> Self {
        Self { help: vec![] }
    }

    pub fn set_help(&mut self, help: Vec<String>) {
        self.help = help;
    }
}

impl Component for ContextHelp {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        if self.is_visible() {
            let help = Paragraph::new(self.help.clone().join(", "))
                .block(main_titled_block("Hints").padding(Padding::new(1, 1, 0, 0)));
            frame.render_widget(help, rect);
        }
    }

    fn is_visible(&self) -> bool {
        true
    }
}

use std::cmp::min;

use ratatui::{
    prelude::{Alignment, CrosstermBackend, Rect},
    style::{Style, Stylize},
    widgets::{block::Title, Block, Borders},
};

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;

pub fn titled_block<'a, T>(title: T) -> Block<'a>
where
    T: Into<Title<'a>>,
{
    Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .title_style(Style::default().bold())
}

pub fn main_titled_block<'a, T>(title: T) -> Block<'a>
where
    T: Into<Title<'a>>,
{
    titled_block(title).borders(Borders::TOP)
}

pub fn calculate_center_rect(width: u16, height: u16, parent_rect: Rect) -> Rect {
    let width = min(width, parent_rect.width);
    let height = min(height, parent_rect.height);
    Rect {
        x: parent_rect.width.saturating_sub(width) / 2,
        y: parent_rect.height.saturating_sub(height) / 2,
        width,
        height,
    }
}

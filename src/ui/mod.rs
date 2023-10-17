use ratatui::{
    prelude::{Alignment, CrosstermBackend, Rect},
    style::{Style, Stylize},
    widgets::{block::Title, Block, Borders, Clear, Paragraph},
};

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;

pub fn main_titled_block<'a, T>(title: T) -> Block<'a>
where
    T: Into<Title<'a>>,
{
    Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::TOP)
        .title_style(Style::default().bold())
}

fn wait_popup<'a>(text: &str) -> Paragraph<'_> {
    Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).on_dark_gray())
}

pub fn draw_wait_popup(text: &str, frame: &mut Frame) {
    let popup = wait_popup(text);

    let width = 30;
    let height = 5;
    let rect = Rect {
        x: frame.size().width.saturating_sub(width) / 2,
        y: frame.size().height.saturating_sub(height) / 2,
        width,
        height,
    };

    frame.render_widget(Clear, rect);
    frame.render_widget(popup, rect);
}

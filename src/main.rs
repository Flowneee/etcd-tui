use std::io::stderr;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::CrosstermBackend;

use self::ui::Ui;

mod ui;

type Terminal = ratatui::Terminal<CrosstermBackend<std::io::Stderr>>;

fn init_terminal() -> Result<Terminal> {
    execute!(stderr(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;
    Ok(terminal)
}

fn cleanup_terminal(terminal: &mut Terminal) -> Result<()> {
    execute!(stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(())
}

fn run(terminal: &mut Terminal) -> Result<()> {
    let mut ui = Ui::new(vec!["key1".into(), "key2".into()]);
    loop {
        terminal.draw(|f| ui.draw(f))?;
        ui.handle_next_event()?;
        if ui.should_quit() {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut terminal = init_terminal()?;

    let result = run(&mut terminal);

    let cleanup_res = cleanup_terminal(&mut terminal);
    result?;
    cleanup_res
}

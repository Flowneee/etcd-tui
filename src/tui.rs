use std::{io::stderr, panic};

use anyhow::Result;
use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::CrosstermBackend;

type Terminal = ratatui::Terminal<CrosstermBackend<std::io::Stderr>>;

pub struct Tui {
    terminal: Terminal,
}

impl Tui {
    pub fn new() -> Result<Self> {
        Ok(Self {
            terminal: Terminal::new(CrosstermBackend::new(stderr()))?,
        })
    }

    pub fn terminal_mut(&mut self) -> &mut Terminal {
        &mut self.terminal
    }

    // copypasta from ratatui book
    pub fn enter(&mut self) -> Result<()> {
        execute!(stderr(), EnterAlternateScreen, cursor::Hide)?;
        enable_raw_mode()?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.clear()?;
        Ok(())
    }

    pub fn reset() -> Result<()> {
        execute!(stderr(), LeaveAlternateScreen, cursor::Show)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        Ok(())
    }
}

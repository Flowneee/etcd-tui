use std::time::Duration;

use anyhow::Result;
use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent};
use futures::StreamExt;
use tokio::time::{interval, MissedTickBehavior};

use crate::SharedState;

macro_rules! key_event {
    ($x:expr) => {
        if matches!($x?, $crate::events::KeyEventState::Consumed) {
            return Ok($crate::events::KeyEventState::Consumed);
        }
    };
}

pub enum KeyEventState {
    Consumed,
    NotConsumed,
}

pub enum Event {
    Tick,
    Keyboard(KeyEvent),
    KeySelected { key: String, value: Option<String> },
    KeyEditDone,
    Tui(CrosstermEvent),
    Quit(Result<()>),
}

pub async fn event_handler(shared_state: SharedState) -> Result<()> {
    let mut tick_interval = interval(Duration::from_millis(100));
    tick_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let mut term_event_stream = EventStream::new().fuse();

    loop {
        let event = tokio::select! {
            _ = tick_interval.tick() => {
                Event::Tick
            }
            ct_event = term_event_stream.select_next_some() => {
                match ct_event? {
                    CrosstermEvent::Key(input) => Event::Keyboard(input),
                    rest => Event::Tui(rest)
                }
            }
        };
        shared_state.send_event(event)?;
    }
}

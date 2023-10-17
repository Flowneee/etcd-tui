use futures::Future;
use tokio::{spawn, sync::oneshot};

use crate::SharedState;

pub struct AsyncTask<T> {
    shared_state: SharedState,
    rx: Option<oneshot::Receiver<T>>,
}

impl<T: Send + 'static> AsyncTask<T> {
    pub fn new(shared_state: SharedState) -> Self {
        Self {
            shared_state,
            rx: None,
        }
    }

    pub fn start<F, Fut>(&mut self, fut_fn: F)
    where
        F: FnOnce(SharedState) -> Fut + Send + 'static,
        Fut: Future<Output = T> + Send,
    {
        let (tx, rx) = oneshot::channel();
        self.rx = Some(rx);
        let shared_state = self.shared_state.clone();

        let _ = spawn(async move {
            let fut = fut_fn(shared_state.clone());
            let result = fut.await;

            let _ = tx.send(result);
            let _ = shared_state.tick();
        });
    }

    pub fn is_active(&self) -> bool {
        self.rx.is_some()
    }

    pub fn try_ready(&mut self) -> Option<T> {
        let rx = self.rx.as_mut()?;
        let x = rx.try_recv().ok()?;
        self.rx = None;
        Some(x)
    }

    pub fn drop_active(&mut self) -> bool {
        self.rx.take().is_some()
    }
}

use futures::Future;
use tokio::{spawn, sync::oneshot, task::JoinHandle};

use crate::SharedState;

pub struct AsyncTask<T> {
    shared_state: SharedState,
    task: Option<(oneshot::Receiver<T>, JoinHandle<()>)>,
}

impl<T> AsyncTask<T>
where
    T: Send + 'static,
{
    pub fn new(shared_state: SharedState) -> Self {
        Self {
            shared_state,
            task: None,
        }
    }

    pub fn start<F, Fut>(&mut self, fut_fn: F)
    where
        F: FnOnce(SharedState) -> Fut + Send + 'static,
        Fut: Future<Output = T> + Send,
    {
        let (tx, rx) = oneshot::channel();
        let shared_state = self.shared_state.clone();

        let handle = spawn(async move {
            let fut = fut_fn(shared_state.clone());
            let result = fut.await;

            let _ = tx.send(result);
            let _ = shared_state.tick();
        });

        self.task = Some((rx, handle));
    }

    pub fn is_active(&self) -> bool {
        self.task.is_some()
    }

    pub fn try_ready(&mut self) -> Option<T> {
        let task = self.task.as_mut()?;
        let x = task.0.try_recv().ok()?;
        self.task = None;
        Some(x)
    }

    /// Abort currently active task.
    ///
    /// If not started, do nothing.
    pub fn abort(&mut self) -> bool {
        if let Some((_, handle)) = self.task.take() {
            handle.abort();
            true
        } else {
            false
        }
    }
}

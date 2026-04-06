//! Provide a handle for canceling asynchronous tasks at any time.

use tokio::task::JoinHandle;

/// A handle to canceling asynchronous tasks at any time， it encapsulates a [`JoinHandle`].
#[derive(Debug)]
pub struct DisposableHandle {
    handle: Option<JoinHandle<()>>,
}

impl DisposableHandle {
    /// constructor
    pub const fn new(handle: JoinHandle<()>) -> Self {
        DisposableHandle {
            handle: Some(handle),
        }
    }

    /// Dispose of the handle.
    pub fn dispose(&mut self) -> bool {
        if let Some(handle) = self.handle.take() {
            handle.abort();
            handle.is_finished()
        } else {
            true
        }
    }
}

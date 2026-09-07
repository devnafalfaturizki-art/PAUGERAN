#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: std::sync::atomic::AtomicBool,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: std::sync::atomic::AtomicBool::new(false),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn child_token(&self) -> Self {
        Self::new()
    }
}

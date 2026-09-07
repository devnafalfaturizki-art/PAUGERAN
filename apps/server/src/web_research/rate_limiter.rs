//! Cooperative rate limiter for outbound research requests.
//!
//! Ensures that PAUGERAN respects server load by spacing requests to
//! the same host with at least one second between calls.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    min_interval: Duration,
    last_request: Mutex<HashMap<String, Instant>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            min_interval: Duration::from_secs(1),
            last_request: Mutex::new(HashMap::new()),
        }
    }
}

impl RateLimiter {
    pub fn with_interval(min_interval: Duration) -> Self {
        Self {
            min_interval,
            last_request: Mutex::new(HashMap::new()),
        }
    }

    pub fn wait_if_needed(&self, host: &str) {
        let mut map = self.last_request.lock().expect("rate limiter mutex");
        let now = Instant::now();
        if let Some(previous) = map.get(host) {
            let elapsed = now.duration_since(*previous);
            if elapsed < self.min_interval {
                let sleep_for = self.min_interval - elapsed;
                std::thread::sleep(sleep_for);
            }
        }
        map.insert(host.to_string(), Instant::now());
    }

    pub fn reset(&self) {
        self.last_request.lock().expect("rate limiter mutex").clear();
    }
}
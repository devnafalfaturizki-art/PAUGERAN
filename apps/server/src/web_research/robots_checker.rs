//! robots.txt compliance checker for outbound research requests.
//!
//! Returns whether a URL is allowed to be fetched based on the host's
//! published robots.txt policy.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct RobotsChecker {
    user_agent: String,
    cache: Mutex<HashMap<String, RobotsPolicy>>,
    ttl: Duration,
}

#[derive(Debug, Clone)]
struct RobotsPolicy {
    allows_all: bool,
    fetched_at: Instant,
    fetch_failed: bool,
}

impl Default for RobotsChecker {
    fn default() -> Self {
        Self {
            user_agent: "PaugeranBot/1.0 (+legal-research)".to_string(),
            cache: Mutex::new(HashMap::new()),
            ttl: Duration::from_secs(60 * 60),
        }
    }
}

impl RobotsChecker {
    pub fn with_user_agent(user_agent: impl Into<String>) -> Self {
        Self {
            user_agent: user_agent.into(),
            cache: Mutex::new(HashMap::new()),
            ttl: Duration::from_secs(60 * 60),
        }
    }

    pub fn is_allowed(&self, url: &str) -> bool {
        let parsed = match url::Url::parse(url) {
            Ok(parsed) => parsed,
            Err(_) => return false,
        };
        let host = match parsed.host_str() {
            Some(host) => host.to_string(),
            None => return false,
        };
        let policy = self.fetch_policy(&host);
        policy.allows_all
    }

    fn fetch_policy(&self, host: &str) -> RobotsPolicy {
        {
            let cache = self.cache.lock().expect("robots cache mutex");
            if let Some(policy) = cache.get(host) {
                if policy.fetched_at.elapsed() < self.ttl {
                    return policy.clone();
                }
            }
        }

        let policy = self.fetch_remote(host);
        let mut cache = self.cache.lock().expect("robots cache mutex");
        cache.insert(host.to_string(), policy.clone());
        policy
    }

    fn fetch_remote(&self, host: &str) -> RobotsPolicy {
        let url = format!("https://{}/robots.txt", host);
        let client = match reqwest::blocking::Client::builder()
            .user_agent(self.user_agent.clone())
            .timeout(Duration::from_secs(5))
            .build()
        {
            Ok(client) => client,
            Err(_) => {
                return RobotsPolicy {
                    allows_all: true,
                    fetched_at: Instant::now(),
                    fetch_failed: true,
                }
            }
        };

        match client.get(&url).send() {
            Ok(response) if response.status().is_success() => {
                if let Ok(text) = response.text() {
                    RobotsPolicy {
                        allows_all: Self::parse_allows(&text, &self.user_agent),
                        fetched_at: Instant::now(),
                        fetch_failed: false,
                    }
                } else {
                    RobotsPolicy {
                        allows_all: true,
                        fetched_at: Instant::now(),
                        fetch_failed: true,
                    }
                }
            }
            _ => RobotsPolicy {
                allows_all: true,
                fetched_at: Instant::now(),
                fetch_failed: true,
            },
        }
    }

    fn parse_allows(content: &str, user_agent: &str) -> bool {
        let mut current_agents: Vec<String> = Vec::new();
        let mut disallowed_paths: Vec<String> = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(rest) = line.strip_prefix("User-agent:") {
                current_agents.push(rest.trim().to_string());
                continue;
            }
            if let Some(rest) = line.strip_prefix("Disallow:") {
                let path = rest.trim();
                if !path.is_empty() && Self::agent_matches(&current_agents, user_agent) {
                    disallowed_paths.push(path.to_string());
                }
                continue;
            }
            if line.starts_with("Allow:") {
                continue;
            }
        }
        disallowed_paths.is_empty()
    }

    fn agent_matches(agents: &[String], user_agent: &str) -> bool {
        agents.iter().any(|agent| agent == "*" || agent == user_agent)
    }
}
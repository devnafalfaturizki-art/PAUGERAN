//! Blocking HTTP client for web research. Wraps reqwest with a
//! shared timeout and cooperative rate limiting.

use std::time::Duration;

use super::rate_limiter::RateLimiter;

pub struct ResearchHttpClient {
    client: reqwest::blocking::Client,
    rate_limiter: RateLimiter,
}

impl Default for ResearchHttpClient {
    fn default() -> Self {
        let client = reqwest::blocking::Client::builder()
            .user_agent("PaugeranBot/1.0 (+legal-research)")
            .timeout(Duration::from_secs(15))
            .build()
            .expect("reqwest client");
        Self {
            client,
            rate_limiter: RateLimiter::default(),
        }
    }
}

impl ResearchHttpClient {
    pub fn fetch(&self, url: &str) -> Result<String, String> {
        let host = url::Url::parse(url)
            .map_err(|e| e.to_string())?
            .host_str()
            .ok_or_else(|| "url tidak memiliki host".to_string())?
            .to_string();
        self.rate_limiter.wait_if_needed(&host);
        let response = self.client.get(url).send().map_err(|e| e.to_string())?;
        if !response.status().is_success() {
            return Err(format!("status tidak berhasil: {}", response.status()));
        }
        response.text().map_err(|e| e.to_string())
    }

    pub fn head(&self, url: &str) -> Result<reqwest::header::HeaderMap, String> {
        let host = url::Url::parse(url)
            .map_err(|e| e.to_string())?
            .host_str()
            .ok_or_else(|| "url tidak memiliki host".to_string())?
            .to_string();
        self.rate_limiter.wait_if_needed(&host);
        let response = self.client.head(url).send().map_err(|e| e.to_string())?;
        Ok(response.headers().clone())
    }
}
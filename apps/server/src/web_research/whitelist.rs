//! Trusted domain whitelist for the legal research module.
//!
//! Only domains in this whitelist may be fetched by PAUGERAN. The
//! whitelist is enforced before any HTTP request is issued and is the
//! single source of truth for outbound research.

use std::collections::HashSet;

pub struct Whitelist {
    domains: HashSet<String>,
}

impl Default for Whitelist {
    fn default() -> Self {
        let mut domains = HashSet::new();
        for domain in [
            "go.id",
            "setkab.go.id",
            "mahkamahagung.go.id",
            "putusan.mahkamahagung.go.id",
            "jdih.kemenkeu.go.id",
            "jdih.kumham.go.id",
            "jdih.setkab.go.id",
            "kemenkumham.go.id",
            "kemenkeu.go.id",
            "bnpp.go.id",
            "bps.go.id",
            "bappenas.go.id",
            "kpk.go.id",
            "kejaksaan.go.id",
            "polri.go.id",
            "kpu.go.id",
            "mkri.go.id",
            "ombudsman.go.id",
            "ojk.go.id",
            "bi.go.id",
            "esdm.go.id",
            "kemenag.go.id",
            "kemenkes.go.id",
            "dephub.go.id",
            "kemenperin.go.id",
            "kementan.go.id",
            "kemenlu.go.id",
            "kemendagri.go.id",
        ] {
            domains.insert(domain.to_string());
        }
        Self { domains }
    }
}

impl Whitelist {
    pub fn allows(&self, url: &str) -> bool {
        match url::Url::parse(url) {
            Ok(parsed) => parsed
                .host_str()
                .map(|host| self.matches(host))
                .unwrap_or(false),
            Err(_) => false,
        }
    }

    pub fn add(&mut self, domain: &str) {
        self.domains.insert(domain.to_lowercase());
    }

    pub fn remove(&mut self, domain: &str) {
        self.domains.remove(domain);
    }

    pub fn domains(&self) -> Vec<String> {
        let mut list: Vec<String> = self.domains.iter().cloned().collect();
        list.sort();
        list
    }

    fn matches(&self, host: &str) -> bool {
        let lower = host.to_lowercase();
        self.domains.contains(&lower)
            || self
                .domains
                .iter()
                .any(|suffix| lower.ends_with(&format!(".{suffix}")))
    }
}
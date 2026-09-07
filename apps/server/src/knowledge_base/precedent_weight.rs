//! Computes precedential weight for Indonesian court decisions using
//! court level, yurisprudensi status, and consistency factors.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecedentWeights {
    pub court_level_score: f32,
    pub consistency_score: f32,
    pub status_score: f32,
    pub temporal_relevance: f32,
}

impl PrecedentWeights {
    pub fn total(&self) -> f32 {
        (self.court_level_score * 0.4
            + self.consistency_score * 0.25
            + self.status_score * 0.2
            + self.temporal_relevance * 0.15)
            .clamp(0.0, 1.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourtLevel {
    DistrictCourt,
    HighCourt,
    SupremeCourt,
    ConstitutionalCourt,
}

impl CourtLevel {
    pub fn score(self) -> f32 {
        match self {
            CourtLevel::DistrictCourt => 0.3,
            CourtLevel::HighCourt => 0.5,
            CourtLevel::SupremeCourt => 0.8,
            CourtLevel::ConstitutionalCourt => 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrecedentStatus {
    Ordinary,
    SupremeCourtDecision,
    YurisprudensiTetap,
    Sema,
}

impl PrecedentStatus {
    pub fn score(self) -> f32 {
        match self {
            PrecedentStatus::Ordinary => 0.3,
            PrecedentStatus::SupremeCourtDecision => 0.7,
            PrecedentStatus::YurisprudensiTetap => 0.95,
            PrecedentStatus::Sema => 0.6,
        }
    }
}

pub struct PrecedentWeightCalculator;

impl PrecedentWeightCalculator {
    pub fn compute(
        level: CourtLevel,
        status: PrecedentStatus,
        consistent_with_majority: bool,
        decision_year: i32,
        reference_year: i32,
    ) -> PrecedentWeights {
        let consistency = if consistent_with_majority { 1.0 } else { 0.3 };
        let age = (reference_year - decision_year).max(0) as f32;
        let temporal_relevance = 1.0 - (age / 30.0).min(1.0);
        PrecedentWeights {
            court_level_score: level.score(),
            consistency_score: consistency,
            status_score: status.score(),
            temporal_relevance,
        }
    }

    pub fn classify_court(name: &str) -> CourtLevel {
        let lower = name.to_lowercase();
        if lower.contains("konstitusi") || lower.contains("mk") {
            CourtLevel::ConstitutionalCourt
        } else if lower.contains("agung") || lower.contains("ma ") {
            CourtLevel::SupremeCourt
        } else if lower.contains("tinggi") {
            CourtLevel::HighCourt
        } else {
            CourtLevel::DistrictCourt
        }
    }
}
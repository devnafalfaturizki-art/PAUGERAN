use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningMode {
    Exploration,
    Preventive,
    Dispute,
    LitigationPrep,
    Adversarial,
    Neutral,
}

impl ReasoningMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exploration => "exploration",
            Self::Preventive => "preventive",
            Self::Dispute => "dispute",
            Self::LitigationPrep => "litigation_prep",
            Self::Adversarial => "adversarial",
            Self::Neutral => "neutral",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "exploration" => Some(Self::Exploration),
            "preventive" => Some(Self::Preventive),
            "dispute" => Some(Self::Dispute),
            "litigation_prep" => Some(Self::LitigationPrep),
            "adversarial" => Some(Self::Adversarial),
            "neutral" => Some(Self::Neutral),
            _ => None,
        }
    }

    pub fn opening_questions(self) -> &'static [&'static str] {
        match self {
            Self::Exploration => &["Apa tujuan utama Anda?", "Kapan fakta utama terjadi?"],
            Self::Preventive => &["Klausul atau tindakan apa yang ingin diamankan?"],
            Self::Dispute => &[
                "Apa tuntutan pihak lawan?",
                "Bukti apa yang sudah tersedia?",
            ],
            Self::LitigationPrep => &["Forum atau prosedur apa yang sudah dipilih?"],
            Self::Adversarial => &["Apa kelemahan paling mungkin diserang lawan?"],
            Self::Neutral => &["Bukti apa yang mendukung masing-masing pihak?"],
        }
    }

    pub fn initial_certainty(self) -> f32 {
        match self {
            Self::Exploration => 0.25,
            Self::Preventive => 0.35,
            Self::Dispute => 0.4,
            Self::LitigationPrep => 0.45,
            Self::Adversarial => 0.3,
            Self::Neutral => 0.3,
        }
    }
}

impl std::str::FromStr for ReasoningMode {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::ReasoningMode;

    #[test]
    fn round_trips_api_names() {
        for mode in [
            ReasoningMode::Exploration,
            ReasoningMode::Preventive,
            ReasoningMode::Dispute,
            ReasoningMode::LitigationPrep,
            ReasoningMode::Adversarial,
            ReasoningMode::Neutral,
        ] {
            assert_eq!(ReasoningMode::parse(mode.as_str()), Some(mode));
        }
    }

    #[test]
    fn exploration_has_two_required_questions() {
        assert_eq!(ReasoningMode::Exploration.opening_questions().len(), 2);
    }
}

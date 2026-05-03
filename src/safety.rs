use crate::model::SafetyAssessment;

pub fn assess_reflection(text: &str) -> SafetyAssessment {
    let normalized = text.to_lowercase();
    let crisis_terms = [
        "hurt myself",
        "kill myself",
        "suicide",
        "end my life",
        "cannot keep myself safe",
        "can't keep myself safe",
        "not safe tonight",
    ];
    let needs_human_help = crisis_terms.iter().any(|term| normalized.contains(term));

    if needs_human_help {
        SafetyAssessment {
            needs_human_help: true,
            banner: "This may need immediate human support. If there is any chance of danger tonight, contact someone nearby now or call emergency support.".to_string(),
            resources: vec![
                "US Suicide & Crisis Lifeline: call or text 988".to_string(),
                "MIT Police / emergency: call 617-253-1212 or 911".to_string(),
                "MIT Student Mental Health & Counseling: reach out for same-day support".to_string(),
            ],
        }
    } else {
        SafetyAssessment {
            needs_human_help: false,
            banner: "Reflection mode only: this app surfaces patterns and low-risk experiments, but it does not provide clinical care.".to_string(),
            resources: vec![
                "For urgent concerns, contact a trusted person or campus support.".to_string(),
                "For immediate danger in the US, call 911 or 988 depending on the situation.".to_string(),
            ],
        }
    }
}

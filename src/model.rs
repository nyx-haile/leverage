use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersonalRecord {
    pub persona: Persona,
    pub check_ins: Vec<CheckIn>,
    pub calendar: Vec<CalendarEvent>,
    pub passive_signals: Vec<PassiveSignal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Persona {
    pub name: String,
    pub description: String,
    pub goal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckIn {
    pub date: String,
    pub stress: f64,
    pub energy: f64,
    pub sleep_hours: f64,
    pub workload: f64,
    pub meals: f64,
    pub social_minutes: f64,
    pub reflection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CalendarEvent {
    pub date: String,
    pub title: String,
    pub kind: EventKind,
    pub pressure: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventKind {
    Deadline,
    Exam,
    Class,
    OfficeHours,
    Recovery,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PassiveSignal {
    pub date: String,
    pub screen_minutes: f64,
    pub steps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalysisReport {
    pub sample_size: usize,
    pub correlations: Vec<Correlation>,
    pub insights: Vec<Insight>,
    pub disclaimer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Correlation {
    pub feature: String,
    pub coefficient: f64,
    pub method: String,
    pub evidence_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Insight {
    pub claim: String,
    pub confidence: Confidence,
    pub evidence: Vec<String>,
    pub humane_next_action: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Confidence {
    High,
    Moderate,
    Exploratory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SafetyAssessment {
    pub needs_human_help: bool,
    pub banner: String,
    pub resources: Vec<String>,
}

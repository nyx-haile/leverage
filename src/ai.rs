use crate::model::{AnalysisReport, Insight};

pub trait InsightNarrator {
    fn summarize(&self, report: &AnalysisReport) -> String;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct MockNarrator;

impl InsightNarrator for MockNarrator {
    fn summarize(&self, report: &AnalysisReport) -> String {
        let Some(primary) = report.insights.first() else {
            return "No stable pattern yet. Add more check-ins before trusting the analysis."
                .to_string();
        };

        summarize_insight(primary)
    }
}

fn summarize_insight(insight: &Insight) -> String {
    format!(
        "Evidence-first summary: {} Next experiment: {}",
        insight.claim, insight.humane_next_action
    )
}

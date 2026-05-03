use crate::model::{AnalysisReport, Confidence, Correlation, Insight, PersonalRecord};

const DISCLAIMER: &str = "Correlation is not diagnosis, causation, or treatment. Stress Reflection Lab shows patterns in your own data so you can choose the next humane step with support from real people.";

pub fn analyze_record(record: &PersonalRecord) -> AnalysisReport {
    let stress: Vec<f64> = record.check_ins.iter().map(|entry| entry.stress).collect();
    let mut correlations = feature_series(record)
        .into_iter()
        .filter_map(|feature| correlation_for(&feature.name, &stress, &feature.values))
        .collect::<Vec<_>>();

    correlations.sort_by(|left, right| {
        right
            .coefficient
            .abs()
            .partial_cmp(&left.coefficient.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let insights = correlations
        .iter()
        .take(3)
        .map(|correlation| insight_for(record, correlation))
        .collect();

    AnalysisReport {
        sample_size: record.check_ins.len(),
        correlations,
        insights,
        disclaimer: DISCLAIMER.to_string(),
    }
}

fn correlation_for(name: &str, stress: &[f64], values: &[f64]) -> Option<Correlation> {
    if stress.len() != values.len()
        || stress.len() < 3
        || is_constant(stress)
        || is_constant(values)
    {
        return None;
    }

    let coefficient = pearson(stress, values)?;
    if !coefficient.is_finite() || coefficient.abs() < 0.25 {
        return None;
    }

    Some(Correlation {
        feature: name.to_string(),
        coefficient,
        method: "Pearson correlation over same-day local check-ins".to_string(),
        evidence_count: stress.len(),
    })
}

fn pearson(left: &[f64], right: &[f64]) -> Option<f64> {
    let mean_left = mean(left)?;
    let mean_right = mean(right)?;
    let mut numerator = 0.0;
    let mut left_sum = 0.0;
    let mut right_sum = 0.0;

    for (left_value, right_value) in left.iter().zip(right.iter()) {
        let left_delta = left_value - mean_left;
        let right_delta = right_value - mean_right;
        numerator += left_delta * right_delta;
        left_sum += left_delta.powi(2);
        right_sum += right_delta.powi(2);
    }

    let denominator = left_sum.sqrt() * right_sum.sqrt();
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        None
    } else {
        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

fn is_constant(values: &[f64]) -> bool {
    values.first().is_none_or(|first| {
        values
            .iter()
            .all(|value| (value - first).abs() < f64::EPSILON)
    })
}

fn insight_for(record: &PersonalRecord, correlation: &Correlation) -> Insight {
    let direction = if correlation.coefficient >= 0.0 {
        "rose with"
    } else {
        "fell as"
    };
    let confidence = match correlation.coefficient.abs() {
        value if value >= 0.70 => Confidence::High,
        value if value >= 0.45 => Confidence::Moderate,
        _ => Confidence::Exploratory,
    };

    Insight {
        claim: format!(
            "Your stress {direction} {} in this local sample (r={:.2}).",
            correlation.feature, correlation.coefficient
        ),
        confidence,
        evidence: evidence_lines(record, &correlation.feature),
        humane_next_action: next_action_for(&correlation.feature),
    }
}

fn evidence_lines(record: &PersonalRecord, feature: &str) -> Vec<String> {
    let mut rows = record
        .check_ins
        .iter()
        .map(|check_in| {
            let feature_value = match feature {
                "sleep debt" => (8.0 - check_in.sleep_hours).max(0.0),
                "deadline pressure" => deadline_pressure(record, &check_in.date),
                "workload" => check_in.workload,
                "screen time" => {
                    signal_value(record, &check_in.date, |signal| signal.screen_minutes)
                }
                "low social time" => (60.0 - check_in.social_minutes).max(0.0),
                "missed meals" => (3.0 - check_in.meals).max(0.0),
                _ => 0.0,
            };
            (check_in.date.clone(), check_in.stress, feature_value)
        })
        .collect::<Vec<_>>();

    rows.sort_by(|left, right| {
        right
            .1
            .partial_cmp(&left.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                right
                    .2
                    .partial_cmp(&left.2)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });

    rows.into_iter()
        .take(3)
        .map(|(date, stress, feature_value)| {
            format!("{date}: stress {stress:.0}, {feature} {feature_value:.1}")
        })
        .collect()
}

fn next_action_for(feature: &str) -> String {
    match feature {
        "sleep debt" => "Treat sleep as the first experiment: protect one 45-minute wind-down block tonight and compare tomorrow's check-in.".to_string(),
        "deadline pressure" => "Pick one deadline and ask a human for scope help: office hours, TA clarification, or a teammate check-in.".to_string(),
        "workload" => "Make the invisible load visible: choose the smallest task that reduces tomorrow's pressure.".to_string(),
        "screen time" => "Try a one-block focus experiment and record whether stress changes afterward.".to_string(),
        "low social time" => "Send one low-stakes check-in message to a friend before the next work block.".to_string(),
        "missed meals" => "Make food non-negotiable for one day and see whether the evening stress score changes.".to_string(),
        _ => "Choose one low-risk experiment and check whether the data changes tomorrow.".to_string(),
    }
}

struct FeatureSeries {
    name: String,
    values: Vec<f64>,
}

fn feature_series(record: &PersonalRecord) -> Vec<FeatureSeries> {
    vec![
        series(record, "sleep debt", |_, _, check_in| {
            (8.0 - check_in.sleep_hours).max(0.0)
        }),
        series(record, "deadline pressure", deadline_pressure_for_day),
        series(record, "workload", |_, _, check_in| check_in.workload),
        series(record, "screen time", |record, date, _| {
            signal_value(record, date, |signal| signal.screen_minutes)
        }),
        series(record, "low social time", |_, _, check_in| {
            (60.0 - check_in.social_minutes).max(0.0)
        }),
        series(record, "missed meals", |_, _, check_in| {
            (3.0 - check_in.meals).max(0.0)
        }),
    ]
}

fn series(
    record: &PersonalRecord,
    name: &str,
    extractor: impl Fn(&PersonalRecord, &str, &crate::model::CheckIn) -> f64,
) -> FeatureSeries {
    FeatureSeries {
        name: name.to_string(),
        values: record
            .check_ins
            .iter()
            .map(|check_in| extractor(record, &check_in.date, check_in))
            .collect(),
    }
}

fn deadline_pressure_for_day(
    record: &PersonalRecord,
    date: &str,
    _: &crate::model::CheckIn,
) -> f64 {
    deadline_pressure(record, date)
}

fn deadline_pressure(record: &PersonalRecord, date: &str) -> f64 {
    record
        .calendar
        .iter()
        .filter(|event| event.date == date)
        .map(|event| event.pressure.max(0.0))
        .sum()
}

fn signal_value(
    record: &PersonalRecord,
    date: &str,
    extractor: impl Fn(&crate::model::PassiveSignal) -> f64,
) -> f64 {
    record
        .passive_signals
        .iter()
        .find(|signal| signal.date == date)
        .map(extractor)
        .unwrap_or(0.0)
}

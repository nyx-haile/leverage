use approx::assert_relative_eq;
use stress_reflection_lab::{analysis::analyze_record, demo, safety::assess_reflection};

#[test]
fn ranks_sleep_debt_and_deadline_pressure_as_top_stress_correlates() {
    let record = demo::overloaded_undergrad_record();
    let report = analyze_record(&record);

    assert!(report.sample_size >= 7);
    assert_eq!(report.correlations[0].feature, "sleep debt");
    assert_eq!(report.correlations[1].feature, "deadline pressure");
    assert!(report.correlations[0].coefficient > 0.75);
    assert!(report.correlations[1].coefficient > 0.65);
    assert!(
        report.insights[0]
            .evidence
            .iter()
            .any(|line| line.contains("2026-05-01"))
    );
    assert!(report.insights[0].claim.contains("sleep debt"));
    assert!(report.disclaimer.contains("Correlation is not diagnosis"));
}

#[test]
fn omits_constant_features_instead_of_returning_nan_correlations() {
    let record = demo::steady_week_record();
    let report = analyze_record(&record);

    assert!(
        report
            .correlations
            .iter()
            .all(|correlation| correlation.coefficient.is_finite())
    );
    assert!(!report.correlations.is_empty());
    assert!(
        !report
            .correlations
            .iter()
            .any(|correlation| correlation.feature == "missed meals")
    );
    assert!(
        report
            .insights
            .iter()
            .all(|insight| !insight.claim.contains("meals"))
    );
}

#[test]
fn computes_pearson_correlation_for_visible_math() {
    let record = demo::overloaded_undergrad_record();
    let report = analyze_record(&record);
    let sleep_debt = report
        .correlations
        .iter()
        .find(|correlation| correlation.feature == "sleep debt")
        .expect("sleep debt correlation should be present");

    assert_relative_eq!(sleep_debt.coefficient, 0.88, epsilon = 0.08);
    assert!(sleep_debt.method.contains("Pearson"));
    assert!(sleep_debt.evidence_count >= 7);
}

#[test]
fn safety_assessment_flags_crisis_language_without_clinical_claims() {
    let assessment =
        assess_reflection("I cannot keep myself safe tonight and I might hurt myself.");

    assert!(assessment.needs_human_help);
    assert!(assessment.banner.contains("immediate human support"));
    assert!(
        assessment
            .resources
            .iter()
            .any(|resource| resource.contains("988"))
    );
    assert!(!assessment.banner.to_lowercase().contains("diagnosis"));
}

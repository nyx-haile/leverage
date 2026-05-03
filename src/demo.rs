use crate::model::{CalendarEvent, CheckIn, EventKind, PassiveSignal, Persona, PersonalRecord};

pub fn overloaded_undergrad_record() -> PersonalRecord {
    PersonalRecord {
        persona: Persona {
            name: "Maya".to_string(),
            description:
                "Overloaded MIT undergrad balancing 6.006, 18.03, a lab class, and a club deadline."
                    .to_string(),
            goal: "Notice stress patterns early enough to ask for help before the week collapses."
                .to_string(),
        },
        check_ins: vec![
            check_in(
                "2026-04-25",
                3.0,
                7.0,
                7.8,
                4.0,
                3.0,
                80.0,
                "Pretty normal day. I studied after brunch and saw friends.",
            ),
            check_in(
                "2026-04-26",
                4.0,
                6.0,
                6.8,
                6.0,
                1.0,
                15.0,
                "Pset started to feel real, but still manageable.",
            ),
            check_in(
                "2026-04-27",
                5.0,
                5.0,
                5.8,
                3.0,
                3.0,
                60.0,
                "I kept switching between classes and did not finish much.",
            ),
            check_in(
                "2026-04-28",
                6.0,
                4.0,
                6.4,
                7.0,
                2.0,
                20.0,
                "Deadline stack is building. I skipped dinner and kept debugging.",
            ),
            check_in(
                "2026-04-29",
                5.0,
                5.0,
                6.9,
                5.0,
                3.0,
                70.0,
                "Still tense, but office hours helped a little.",
            ),
            check_in(
                "2026-04-30",
                8.0,
                2.0,
                5.0,
                8.0,
                2.0,
                35.0,
                "Pulled a late night. I am behind and everything feels urgent.",
            ),
            check_in(
                "2026-05-01",
                9.0,
                1.0,
                5.2,
                6.0,
                2.0,
                25.0,
                "Exam tomorrow, lab due, and I have not asked anyone for help.",
            ),
            check_in(
                "2026-05-02",
                7.0,
                3.0,
                6.0,
                7.0,
                1.0,
                55.0,
                "Slept a bit more but still feel the aftershock from the deadline pileup.",
            ),
        ],
        calendar: vec![
            event(
                "2026-04-26",
                "6.006 pset checkpoint",
                EventKind::Deadline,
                0.5,
            ),
            event("2026-04-28", "Lab notebook due", EventKind::Deadline, 1.2),
            event(
                "2026-04-29",
                "18.03 office hours",
                EventKind::OfficeHours,
                -0.3,
            ),
            event("2026-04-30", "6.006 pset due", EventKind::Deadline, 1.8),
            event("2026-05-01", "18.03 exam", EventKind::Exam, 2.2),
            event("2026-05-01", "Lab report due", EventKind::Deadline, 1.6),
            event(
                "2026-05-02",
                "Club launch deadline",
                EventKind::Deadline,
                1.0,
            ),
        ],
        passive_signals: vec![
            signal("2026-04-25", 210.0, 9000.0),
            signal("2026-04-26", 500.0, 7600.0),
            signal("2026-04-27", 240.0, 6200.0),
            signal("2026-04-28", 420.0, 4000.0),
            signal("2026-04-29", 360.0, 5200.0),
            signal("2026-04-30", 330.0, 2500.0),
            signal("2026-05-01", 610.0, 1800.0),
            signal("2026-05-02", 300.0, 3100.0),
        ],
    }
}

pub fn steady_week_record() -> PersonalRecord {
    PersonalRecord {
        persona: Persona {
            name: "Jordan".to_string(),
            description: "Student with a steady week and intentionally constant meal counts."
                .to_string(),
            goal: "Avoid inventing patterns when the data does not support them.".to_string(),
        },
        check_ins: vec![
            check_in("2026-04-25", 4.0, 6.0, 7.0, 4.0, 3.0, 30.0, "Steady day."),
            check_in("2026-04-26", 4.0, 6.0, 7.0, 4.0, 3.0, 30.0, "Steady day."),
            check_in(
                "2026-04-27",
                5.0,
                5.0,
                6.8,
                5.0,
                3.0,
                25.0,
                "Slightly busier.",
            ),
            check_in(
                "2026-04-28",
                5.0,
                5.0,
                6.8,
                5.0,
                3.0,
                25.0,
                "Slightly busier.",
            ),
            check_in("2026-04-29", 4.0, 6.0, 7.2, 4.0, 3.0, 35.0, "Recovered."),
        ],
        calendar: vec![],
        passive_signals: vec![],
    }
}

fn check_in(
    date: &str,
    stress: f64,
    energy: f64,
    sleep_hours: f64,
    workload: f64,
    meals: f64,
    social_minutes: f64,
    reflection: &str,
) -> CheckIn {
    CheckIn {
        date: date.to_string(),
        stress,
        energy,
        sleep_hours,
        workload,
        meals,
        social_minutes,
        reflection: reflection.to_string(),
    }
}

fn event(date: &str, title: &str, kind: EventKind, pressure: f64) -> CalendarEvent {
    CalendarEvent {
        date: date.to_string(),
        title: title.to_string(),
        kind,
        pressure,
    }
}

fn signal(date: &str, screen_minutes: f64, steps: f64) -> PassiveSignal {
    PassiveSignal {
        date: date.to_string(),
        screen_minutes,
        steps,
    }
}

use std::{path::PathBuf, time::Duration};

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Tabs, Wrap},
};
use stress_reflection_lab::{
    ai::{InsightNarrator, MockNarrator},
    analysis::analyze_record,
    demo,
    model::{AnalysisReport, Confidence, PersonalRecord, SafetyAssessment},
    safety::assess_reflection,
    store,
};

const DEMO_RECORD_PATH: &str = "target/stress-reflection-lab/overloaded-undergrad.json";

pub fn run() -> Result<()> {
    let context = DemoContext::build()?;
    let mut app = App::new(context);
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal, &mut app);
    ratatui::restore();
    result
}

pub fn demo_report() -> Result<String> {
    let context = DemoContext::build()?;
    Ok(render_report(&context))
}

fn run_app(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| render(frame, app))?;
        if event::poll(Duration::from_millis(theme::TICK_MS))? {
            handle_event(app, event::read()?)?;
        }
    }

    Ok(())
}

fn handle_event(app: &mut App, event: Event) -> Result<()> {
    if let Event::Key(key) = event {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
            KeyCode::Char('1') => app.screen = Screen::Overview,
            KeyCode::Char('2') => app.screen = Screen::Evidence,
            KeyCode::Char('3') => app.screen = Screen::Safety,
            KeyCode::Right | KeyCode::Char('l') => app.next_screen(),
            KeyCode::Left | KeyCode::Char('h') => app.previous_screen(),
            _ => {}
        }
    }

    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    let [header_area, tabs_area, body_area, help_area] = Layout::vertical([
        Constraint::Length(theme::HEADER_HEIGHT),
        Constraint::Length(theme::TABS_HEIGHT),
        Constraint::Min(theme::BODY_MIN_HEIGHT),
        Constraint::Length(theme::HELP_HEIGHT),
    ])
    .areas(frame.area());

    render_header(frame, header_area, &app.context);
    render_tabs(frame, tabs_area, app.screen);
    match app.screen {
        Screen::Overview => render_overview(frame, body_area, &app.context),
        Screen::Evidence => render_evidence(frame, body_area, &app.context),
        Screen::Safety => render_safety(frame, body_area, &app.context),
    }
    render_help(frame, help_area);
}

fn render_header(frame: &mut Frame, area: Rect, context: &DemoContext) {
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Stress Reflection Lab", theme::title()),
            Span::raw("  "),
            Span::styled("a data scientist for your life", theme::accent()),
        ]),
        Line::from(vec![
            Span::styled("Persona: ", theme::label()),
            Span::raw(context.record.persona.name.as_str()),
            Span::raw(" - "),
            Span::raw(context.record.persona.description.as_str()),
        ]),
    ])
    .block(theme::panel("Local demo"))
    .wrap(Wrap { trim: true });
    frame.render_widget(header, area);
}

fn render_tabs(frame: &mut Frame, area: Rect, screen: Screen) {
    let tabs = Tabs::new(["1 Overview", "2 Evidence", "3 Safety"])
        .select(screen.index())
        .style(theme::muted())
        .highlight_style(theme::tab_active())
        .divider(" | ");
    frame.render_widget(tabs, area);
}

fn render_overview(frame: &mut Frame, area: Rect, context: &DemoContext) {
    let [left, right] = Layout::horizontal([
        Constraint::Percentage(theme::OVERVIEW_LEFT_PERCENT),
        Constraint::Percentage(theme::OVERVIEW_RIGHT_PERCENT),
    ])
    .areas(area);
    let [insight_area, privacy_area] = Layout::vertical([
        Constraint::Percentage(theme::INSIGHT_PERCENT),
        Constraint::Percentage(theme::PRIVACY_PERCENT),
    ])
    .areas(left);

    let primary = context.primary_insight();
    let insight_lines = vec![
        Line::from(Span::styled("Evidence-first insight", theme::label())),
        Line::from(context.narration.as_str()),
        Line::raw(""),
        Line::from(vec![
            Span::styled("Confidence: ", theme::label()),
            Span::raw(confidence_label(primary.confidence)),
        ]),
        Line::from(vec![
            Span::styled("Next humane experiment: ", theme::label()),
            Span::raw(primary.humane_next_action.as_str()),
        ]),
    ];
    let insight = Paragraph::new(insight_lines)
        .block(theme::panel("Judge-ready headline"))
        .wrap(Wrap { trim: true });
    frame.render_widget(insight, insight_area);

    let privacy = Paragraph::new(vec![
        Line::from(
            "Manual check-ins first; calendar and passive signals are clearly second-tier hooks.",
        ),
        Line::from("The demo round-trips Maya's JSON through local storage only."),
        Line::from("No diagnosis, no therapy, no autonomous action, no cloud account."),
    ])
    .block(theme::panel("Local-first frame"))
    .wrap(Wrap { trim: true });
    frame.render_widget(privacy, privacy_area);

    render_correlation_table(frame, right, context, theme::CORRELATION_ROWS_OVERVIEW);
}

fn render_evidence(frame: &mut Frame, area: Rect, context: &DemoContext) {
    let [table_area, rows_area] = Layout::vertical([
        Constraint::Length(theme::EVIDENCE_TABLE_HEIGHT),
        Constraint::Min(theme::BODY_MIN_HEIGHT),
    ])
    .areas(area);

    render_correlation_table(frame, table_area, context, theme::CORRELATION_ROWS_EVIDENCE);

    let mut lines = Vec::new();
    for (index, insight) in context.report.insights.iter().enumerate() {
        lines.push(Line::from(vec![
            Span::styled(format!("Insight {}: ", index + 1), theme::label()),
            Span::raw(insight.claim.as_str()),
        ]));
        for evidence in &insight.evidence {
            lines.push(Line::from(vec![
                Span::raw("  - "),
                Span::raw(evidence.as_str()),
            ]));
        }
        lines.push(Line::raw(""));
    }

    let evidence = Paragraph::new(lines)
        .block(theme::panel("Evidence rows from Maya's week"))
        .wrap(Wrap { trim: true });
    frame.render_widget(evidence, rows_area);
}

fn render_safety(frame: &mut Frame, area: Rect, context: &DemoContext) {
    let [safety_area, science_area] = Layout::vertical([
        Constraint::Percentage(theme::SAFETY_COPY_PERCENT),
        Constraint::Percentage(theme::SCIENCE_COPY_PERCENT),
    ])
    .areas(area);

    let mut safety_lines = vec![
        Line::from(context.safety.banner.as_str()),
        Line::raw(""),
        Line::from("Privacy posture:"),
        Line::from("  - Data stays on this machine for the prototype."),
        Line::from("  - Mock AI summarizes computed correlations; it does not infer diagnoses."),
        Line::from("  - The product suggests low-risk reflection experiments, not medical care."),
    ];
    safety_lines.push(Line::raw(""));
    safety_lines.push(Line::from("Human support copy:"));
    for resource in &context.safety.resources {
        safety_lines.push(Line::from(vec![
            Span::raw("  - "),
            Span::raw(resource.as_str()),
        ]));
    }

    let safety = Paragraph::new(safety_lines)
        .block(theme::panel("Safety and privacy"))
        .wrap(Wrap { trim: true });
    frame.render_widget(safety, safety_area);

    let science = Paragraph::new(vec![
        Line::from("Science stance for the pitch:"),
        Line::from(context.report.disclaimer.as_str()),
        Line::raw(""),
        Line::from("What judges should see: verified local analysis, transparent evidence rows, and a humane next step Maya controls."),
    ])
    .block(theme::panel("No clinical claims"))
    .wrap(Wrap { trim: true });
    frame.render_widget(science, science_area);
}

fn render_correlation_table(
    frame: &mut Frame,
    area: Rect,
    context: &DemoContext,
    row_limit: usize,
) {
    let rows = context
        .report
        .correlations
        .iter()
        .take(row_limit)
        .map(|correlation| {
            Row::new([
                Cell::from(correlation.feature.as_str()),
                Cell::from(format_coefficient(correlation.coefficient)),
                Cell::from(confidence_from_coefficient(correlation.coefficient)),
                Cell::from(correlation.evidence_count.to_string()),
                Cell::from(correlation.method.as_str()),
            ])
        });

    let table = Table::new(
        rows,
        [
            Constraint::Length(theme::FEATURE_COLUMN_WIDTH),
            Constraint::Length(theme::R_COLUMN_WIDTH),
            Constraint::Length(theme::CONFIDENCE_COLUMN_WIDTH),
            Constraint::Length(theme::N_COLUMN_WIDTH),
            Constraint::Min(theme::METHOD_COLUMN_MIN_WIDTH),
        ],
    )
    .header(
        Row::new(["Feature", "r", "Confidence", "n", "Method"])
            .style(theme::table_header())
            .bottom_margin(theme::TABLE_HEADER_MARGIN),
    )
    .block(theme::panel("Correlation table"))
    .column_spacing(theme::TABLE_COLUMN_SPACING)
    .row_highlight_style(theme::table_highlight());
    frame.render_widget(
        table,
        area.inner(Margin::new(theme::PANEL_INSET_X, theme::PANEL_INSET_Y)),
    );
}

fn render_help(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new(
        "Keys: 1 Overview | 2 Evidence | 3 Safety | Left/Right or h/l Switch | q Quit",
    )
    .style(theme::help())
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(theme::muted()),
    );
    frame.render_widget(help, area);
}

fn render_report(context: &DemoContext) -> String {
    let primary = context.primary_insight();
    let mut output = String::new();
    output.push_str("Stress Reflection Lab demo report\n");
    output.push_str("Headline: a data scientist for your life\n\n");
    output.push_str("Persona\n");
    output.push_str(&format!(
        "- {}: {}\n",
        context.record.persona.name, context.record.persona.description
    ));
    output.push_str(&format!("- Goal: {}\n\n", context.record.persona.goal));
    output.push_str("Evidence-first insight\n");
    output.push_str(&format!("- {}\n", context.narration));
    output.push_str(&format!(
        "- Confidence: {}\n",
        confidence_label(primary.confidence)
    ));
    output.push_str(&format!(
        "- Next humane experiment: {}\n\n",
        primary.humane_next_action
    ));
    output.push_str("Correlation table\n");
    output.push_str("Feature | r | Confidence | n | Method\n");
    for correlation in &context.report.correlations {
        output.push_str(&format!(
            "{} | {} | {} | {} | {}\n",
            correlation.feature,
            format_coefficient(correlation.coefficient),
            confidence_from_coefficient(correlation.coefficient),
            correlation.evidence_count,
            correlation.method
        ));
    }
    output.push_str("\nEvidence rows\n");
    for insight in &context.report.insights {
        output.push_str(&format!("- {}\n", insight.claim));
        for evidence in &insight.evidence {
            output.push_str(&format!("  - {}\n", evidence));
        }
    }
    output.push_str("\nSafety and privacy\n");
    output.push_str(&format!("- {}\n", context.safety.banner));
    output.push_str("- Local-first prototype: manual check-ins first, calendar/passive signals second-tier, no cloud storage.\n");
    output.push_str("- Not therapy, not diagnosis, not treatment, and no autonomous actions.\n");
    output.push_str(&format!("- {}\n", context.report.disclaimer));
    output.push_str("\nKeyboard help\n");
    output.push_str(
        "- 1 Overview, 2 Evidence, 3 Safety, Left/Right or h/l switch screens, q quit.\n",
    );
    output
}

fn format_coefficient(coefficient: f64) -> String {
    format!("{coefficient:.2}")
}

fn confidence_from_coefficient(coefficient: f64) -> &'static str {
    match coefficient.abs() {
        value if value >= theme::HIGH_CONFIDENCE_THRESHOLD => "High",
        value if value >= theme::MODERATE_CONFIDENCE_THRESHOLD => "Moderate",
        _ => "Exploratory",
    }
}

fn confidence_label(confidence: Confidence) -> &'static str {
    match confidence {
        Confidence::High => "High",
        Confidence::Moderate => "Moderate",
        Confidence::Exploratory => "Exploratory",
    }
}

#[derive(Debug)]
struct DemoContext {
    record: PersonalRecord,
    report: AnalysisReport,
    narration: String,
    safety: SafetyAssessment,
}

impl DemoContext {
    fn build() -> Result<Self> {
        let path = PathBuf::from(DEMO_RECORD_PATH);
        let source_record = demo::overloaded_undergrad_record();
        store::save_record(&path, &source_record).context("failed to save local demo record")?;
        let record = store::load_record(&path).context("failed to reload local demo record")?;
        let report = analyze_record(&record);
        let narration = MockNarrator.summarize(&report);
        let reflection_text = record
            .check_ins
            .iter()
            .map(|entry| entry.reflection.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let safety = assess_reflection(&reflection_text);

        Ok(Self {
            record,
            report,
            narration,
            safety,
        })
    }

    fn primary_insight(&self) -> &stress_reflection_lab::model::Insight {
        self.report
            .insights
            .first()
            .expect("demo record should produce at least one insight")
    }
}

#[derive(Debug)]
struct App {
    context: DemoContext,
    screen: Screen,
    should_quit: bool,
}

impl App {
    fn new(context: DemoContext) -> Self {
        Self {
            context,
            screen: Screen::Overview,
            should_quit: false,
        }
    }

    fn next_screen(&mut self) {
        self.screen = match self.screen {
            Screen::Overview => Screen::Evidence,
            Screen::Evidence => Screen::Safety,
            Screen::Safety => Screen::Overview,
        };
    }

    fn previous_screen(&mut self) {
        self.screen = match self.screen {
            Screen::Overview => Screen::Safety,
            Screen::Evidence => Screen::Overview,
            Screen::Safety => Screen::Evidence,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Overview,
    Evidence,
    Safety,
}

impl Screen {
    fn index(self) -> usize {
        match self {
            Screen::Overview => 0,
            Screen::Evidence => 1,
            Screen::Safety => 2,
        }
    }
}

mod theme {
    use super::*;

    pub const TICK_MS: u64 = 120;
    pub const HEADER_HEIGHT: u16 = 5;
    pub const TABS_HEIGHT: u16 = 2;
    pub const HELP_HEIGHT: u16 = 2;
    pub const BODY_MIN_HEIGHT: u16 = 10;
    pub const OVERVIEW_LEFT_PERCENT: u16 = 52;
    pub const OVERVIEW_RIGHT_PERCENT: u16 = 48;
    pub const INSIGHT_PERCENT: u16 = 62;
    pub const PRIVACY_PERCENT: u16 = 38;
    pub const SAFETY_COPY_PERCENT: u16 = 64;
    pub const SCIENCE_COPY_PERCENT: u16 = 36;
    pub const EVIDENCE_TABLE_HEIGHT: u16 = 11;
    pub const CORRELATION_ROWS_OVERVIEW: usize = 5;
    pub const CORRELATION_ROWS_EVIDENCE: usize = 6;
    pub const FEATURE_COLUMN_WIDTH: u16 = 18;
    pub const R_COLUMN_WIDTH: u16 = 7;
    pub const CONFIDENCE_COLUMN_WIDTH: u16 = 12;
    pub const N_COLUMN_WIDTH: u16 = 5;
    pub const METHOD_COLUMN_MIN_WIDTH: u16 = 24;
    pub const TABLE_COLUMN_SPACING: u16 = 1;
    pub const TABLE_HEADER_MARGIN: u16 = 1;
    pub const PANEL_INSET_X: u16 = 1;
    pub const PANEL_INSET_Y: u16 = 0;
    pub const HIGH_CONFIDENCE_THRESHOLD: f64 = 0.70;
    pub const MODERATE_CONFIDENCE_THRESHOLD: f64 = 0.45;

    const INK: Color = Color::Rgb(235, 232, 218);
    const MUTED: Color = Color::Rgb(151, 158, 150);
    const PANEL: Color = Color::Rgb(92, 107, 92);
    const ACCENT: Color = Color::Rgb(146, 198, 126);
    const GOLD: Color = Color::Rgb(218, 175, 87);
    const BASE: Color = Color::Rgb(18, 23, 20);

    pub fn panel(title: &'static str) -> Block<'static> {
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(PANEL))
            .style(Style::default().fg(INK).bg(BASE))
    }

    pub fn title() -> Style {
        Style::default().fg(INK).add_modifier(Modifier::BOLD)
    }

    pub fn accent() -> Style {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    }

    pub fn label() -> Style {
        Style::default().fg(GOLD).add_modifier(Modifier::BOLD)
    }

    pub fn muted() -> Style {
        Style::default().fg(MUTED)
    }

    pub fn help() -> Style {
        Style::default().fg(INK).bg(BASE)
    }

    pub fn tab_active() -> Style {
        Style::default()
            .fg(BASE)
            .bg(ACCENT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn table_header() -> Style {
        Style::default().fg(GOLD).add_modifier(Modifier::BOLD)
    }

    pub fn table_highlight() -> Style {
        Style::default().fg(BASE).bg(ACCENT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_contains_demo_must_haves() {
        let report = demo_report().expect("demo report should render");

        assert!(report.contains("Maya"));
        assert!(report.contains("MIT undergrad"));
        assert!(report.contains("a data scientist for your life"));
        assert!(report.contains("Correlation table"));
        assert!(report.contains("Evidence rows"));
        assert!(report.contains("Safety and privacy"));
        assert!(report.contains("Not therapy, not diagnosis"));
        assert!(report.contains("Keyboard help"));
    }
}

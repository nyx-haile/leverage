# Stress Reflection Lab

Stress Reflection Lab is a local-first Rust/Ratatui prototype for the Claude Builder Club Spring 2026 Hackathon. It is built for an overloaded MIT undergrad who needs evidence-backed self-reflection before stress turns into a crisis.

Pitch: **a data scientist for your life**. The app turns manual check-ins, mock calendar pressure, and optional passive-signal hooks into transparent correlations, evidence rows, and one low-risk next experiment the student controls.

## Demo

Run the interactive local TUI:

```bash
cargo run
```

Run the non-interactive report for judging, tests, or video narration:

```bash
cargo run -- --demo-report
```

Keyboard controls in the TUI:

- `1`: Overview
- `2`: Evidence
- `3`: Safety
- `Left` / `Right` or `h` / `l`: switch screens
- `q` or `Esc`: quit

## What The Prototype Shows

- A seeded MIT undergrad persona, Maya, with eight days of local check-ins.
- Real Pearson correlations over local data, not vague AI claims.
- Top factors such as sleep debt and deadline pressure, with visible evidence rows.
- A mock AI narrator that summarizes computed results through a pluggable trait.
- Safety copy that keeps the product out of diagnosis, therapy, treatment, and autonomous action.
- Local JSON round-trip under `target/stress-reflection-lab/` to reinforce the privacy posture.

## Architecture

- `src/model.rs`: serializable personal record, check-ins, signals, insights, and safety structs.
- `src/demo.rs`: seeded overloaded-undergrad and steady-week records.
- `src/analysis.rs`: deterministic feature extraction, Pearson correlation, insight generation, and evidence rows.
- `src/safety.rs`: crisis-language guardrail and support resources.
- `src/ai.rs`: pluggable `InsightNarrator` trait with a deterministic mock provider.
- `src/store.rs`: local JSON save/load helpers.
- `src/tui.rs`: Ratatui overview, evidence, safety screens, and demo report renderer.

## Hackathon Framing

Track: **Neuroscience & Mental Health**.

Impact story: stressed students often have scattered signals but no private, humane way to see patterns early. This prototype helps them understand their own data and ask for human help sooner.

Ethical stance: the app does not diagnose, treat, replace therapy, contact others, or take autonomous actions. It surfaces correlations as hypotheses and keeps the student in control.

## Verification

```bash
cargo fmt --check
cargo check
cargo test
cargo run -- --demo-report
```

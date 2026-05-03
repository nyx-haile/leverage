mod tui;

use anyhow::Result;

fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "--demo-report") {
        print!("{}", tui::demo_report()?);
        return Ok(());
    }

    tui::run()
}

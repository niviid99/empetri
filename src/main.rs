use empetri::ui;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

fn main() {
    if log_init().is_err() {
        eprintln!("Failed to init loggerr");
    }

    let natopts = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "empetri",
        natopts,
        Box::new(|_cc| Ok(Box::new(ui::UI::new()))),
    );
}

fn log_init() -> anyhow::Result<()> {
    let config = ConfigBuilder::new().add_filter_allow_str("empetri").build();

    TermLogger::init(
        LevelFilter::Trace,
        config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    Ok(())
}

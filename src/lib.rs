pub mod services;
pub mod ui;
pub mod uistyles;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );

    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "empetri",
        options,
        Box::new(|_cc| Ok(Box::new(ui::UI::new()))),
    );
}

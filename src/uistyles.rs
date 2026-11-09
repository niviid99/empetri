pub fn custom(ui: &mut egui::Ui) {
    let mut vconf = egui::Visuals::light();

    vconf.widgets.noninteractive.corner_radius = egui::CornerRadius::ZERO;
    vconf.widgets.inactive.corner_radius = egui::CornerRadius::ZERO;
    vconf.widgets.hovered.corner_radius = egui::CornerRadius::ZERO;
    vconf.widgets.active.corner_radius = egui::CornerRadius::ZERO;

    vconf.window_corner_radius = egui::CornerRadius::ZERO;

    ui.ctx().set_visuals(vconf);
}

pub fn customfont(ui: &mut egui::Ui) {
    let mut fonts = egui::FontDefinitions::default();

    let _ = fonts.font_data.insert(
        "agave".to_string(),
        std::sync::Arc::new(egui::FontData::from_owned(
            include_bytes!("../agave.ttf").to_vec(),
        )),
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "agave".to_string());

    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "agave".to_string());

    ui.set_fonts(fonts);
}

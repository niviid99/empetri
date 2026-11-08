pub fn custom(ui: &mut egui::Ui) {
    let mut vconf = egui::Visuals::dark();

    vconf.widgets.noninteractive.corner_radius = egui::CornerRadius::ZERO;
    vconf.widgets.inactive.corner_radius = egui::CornerRadius::ZERO;
    vconf.widgets.hovered.corner_radius = egui::CornerRadius::ZERO;
    vconf.widgets.active.corner_radius = egui::CornerRadius::ZERO;

    vconf.window_corner_radius = egui::CornerRadius::ZERO;

    ui.ctx().set_visuals(vconf);
}

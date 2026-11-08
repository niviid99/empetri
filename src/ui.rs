use std::{path::PathBuf, time::Duration};

use crate::{services, uistyles};
use eframe::App;
use egui::Direction::BottomUp;

pub struct UI {
    first_frame: bool,
    filesystem_service: services::FileSystem,
    workdir: String,
    files_list: Vec<String>,
    audio_service: Option<services::AudioService>,
    now_playing: Option<String>,
    track_cursor: u64,
    volume: f32,
    repeat_track: bool,
}

impl UI {
    pub fn new() -> Self {
        Self {
            first_frame: true,
            filesystem_service: services::FileSystem::by_path_str(
                "/data/data/com.niviid99.empetri/files/Music", // THIS IS FUCKIN SHIT!!!!!!!
            ),
            workdir: String::new(),
            files_list: Vec::new(),
            audio_service: None,
            now_playing: None,
            track_cursor: 0,
            volume: 1.0,
            repeat_track: false,
        }
    }
}

impl App for UI {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.first_frame {
            self.first_frame = false;
            uistyles::custom(ui);
            log::info!("Setted up custom ui style");

            self.ff();
            log::info!(
                "Initialised first frame success: workdir = {}",
                self.workdir
            );
        }

        if self.audio_service.is_some() {
            egui::Panel::top("toolbar_top").show(ui, |ui| {
                self.toolbar_top(ui);
            });

            egui::Panel::bottom("toolbar_bottom").show(ui, |ui| {
                self.toolbar_bottom(ui);
            });

            egui::CentralPanel::default().show(ui, |ui| {
                self.central(ui);
            });
        } else {
            if self.error_messagebox(ui, "Audio service is not initialised!", "Try restart") {
                self.start_audioservice();
            }
        }
    }
}

impl UI {
    fn ff(&mut self) {
        self.workdir = self
            .filesystem_service
            .get_current_path()
            .to_string_lossy()
            .to_string();
        self.refresh_files_list();
        self.start_audioservice();
    }

    fn central(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for file in self.files_list.iter() {
                    let selected: bool;
                    if let Some(now) = &self.now_playing {
                        selected = file == now;
                    } else {
                        selected = false;
                    }

                    let file_only_name: String;
                    if let Some(fnameref) = PathBuf::from(&file).file_name() {
                        file_only_name = fnameref.to_string_lossy().to_string();
                    } else {
                        file_only_name = file.clone();
                    }
                    if ui
                        .selectable_label(selected, format!("{}  \u{f075a}", file_only_name))
                        .clicked()
                    {
                        if let Some(audio_service) = &mut self.audio_service {
                            if audio_service.load(file).is_ok() {
                                audio_service.play();
                                self.now_playing = Some(file.clone());
                                self.track_cursor = 0;
                                audio_service.set_pos(Duration::from_secs(self.track_cursor));
                            } else {
                                self.now_playing = None;
                                self.track_cursor = 0;
                                log::error!("Failed to play {}", file);
                            }
                        }
                    }
                }
            });
    }

    fn toolbar_bottom(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if let Some(audio_service) = &mut self.audio_service {
                if audio_service.is_playing() {
                    if ui.button("\u{f04c}").clicked() {
                        audio_service.pause();
                    }
                } else {
                    if ui.button("\u{f04b}").clicked() {
                        audio_service.play();
                    }
                }
                if ui.button("\u{f04d}").clicked() {
                    self.now_playing = None;
                    self.track_cursor = 0;
                    audio_service.stop();
                }
            }

            self.track_controll(ui);
        });
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.repeat_track, "repeat");
            self.volume_controll(ui);
        });
        ui.add_space(50.0);
    }

    fn track_controll(&mut self, ui: &mut egui::Ui) {
        if let Some(audio_service) = &mut self.audio_service {
            if let Some(track_size) = audio_service.get_song_duration() {
                let dursec = track_size.as_secs();
                if audio_service.is_playing() {
                    self.track_cursor = audio_service.get_current().as_secs();
                }
                let oldtp = self.track_cursor;
                ui.add(egui::Slider::new(&mut self.track_cursor, 0..=dursec).show_value(false));
                if oldtp != self.track_cursor {
                    audio_service.set_pos(Duration::from_secs(self.track_cursor));
                }

                if self.track_cursor >= dursec - 5 {
                    if !self.repeat_track {
                        audio_service.stop();
                        self.now_playing = None;
                        self.track_cursor = 0;
                    } else {
                        audio_service.set_pos(Duration::from_secs(0));
                        self.track_cursor = 0;
                    }
                }
            }
        }
    }

    fn volume_controll(&mut self, ui: &mut egui::Ui) {
        if let Some(audio_sevice) = &mut self.audio_service {
            let old_vol = self.volume;
            ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).text("volume"));
            if self.volume != old_vol {
                audio_sevice.set_volume(self.volume);
            }
        }
    }

    fn toolbar_top(&mut self, ui: &mut egui::Ui) {
        ui.add_space(50.0);
        ui.text_edit_singleline(&mut self.workdir);
        if ui.button("refresh").clicked() {
            self.filesystem_service.change_path_str(&self.workdir);
            self.refresh_files_list();
            ui.request_repaint();
        }
    }

    fn refresh_files_list(&mut self) {
        if self
            .filesystem_service
            .files_filtered(&[".mp3", ".flac", ".ogg"], &mut self.files_list)
            .is_err()
        {
            self.files_list.clear();
            log::error!("Failed to read files list in {}", self.workdir);
        } else {
            log::info!("Refreshed files from {} success", self.workdir);
        }
    }

    fn start_audioservice(&mut self) {
        log::info!("Restarting audio service");
        self.audio_service = None;
        if let Ok(aus) = services::AudioService::new() {
            self.audio_service = Some(aus);
            log::info!("Audio service started");
        } else {
            log::error!("Failed to start audio service")
        }
    }

    fn error_messagebox(&self, ui: &mut egui::Ui, text: &str, button_text: &str) -> bool {
        let mut pressed = false;
        egui::Window::new("Error").resizable(false).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(text);
                pressed = ui.button(button_text).clicked();
            });
        });
        pressed
    }
}

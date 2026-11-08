use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use std::{fs::File, path::PathBuf, time::Duration};
use walkdir::WalkDir;

pub struct AudioService {
    _device: MixerDeviceSink,
    player: Player,
    track_duration: Option<Duration>,
}
impl AudioService {
    pub fn new() -> anyhow::Result<Self> {
        let _device = DeviceSinkBuilder::open_default_sink()?;
        let player = Player::connect_new(&_device.mixer());

        Ok(Self {
            _device,
            player,
            track_duration: None,
        })
    }

    pub fn play(&mut self) {
        self.player.play();
        log::info!("Playing");
    }

    pub fn load(&mut self, filename: &str) -> anyhow::Result<()> {
        self.player.stop();
        self.player.clear();

        let file = File::open(filename)?;
        let source = Decoder::try_from(file)?;
        self.track_duration = source.total_duration();
        self.player.append(source);

        log::info!("Loaded {}", filename);

        Ok(())
    }

    pub fn is_playing(&self) -> bool {
        !self.player.is_paused()
    }

    pub fn pause(&self) {
        self.player.pause();
        log::info!("Paused");
    }

    pub fn stop(&mut self) {
        self.player.stop();
        self.track_duration = None;
        log::info!("Stoped");
    }

    pub fn get_song_duration(&self) -> Option<Duration> {
        self.track_duration
    }

    pub fn get_current(&self) -> Duration {
        self.player.get_pos()
    }

    pub fn set_pos(&mut self, dur: Duration) {
        let _ = self.player.try_seek(dur);
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.player.set_volume(volume);
    }
}

pub struct FileSystem {
    workdir: PathBuf,
}

impl FileSystem {
    pub fn from_exec() -> Self {
        let current_path = std::env::current_dir().unwrap_or(PathBuf::from("/"));
        Self {
            workdir: current_path,
        }
    }

    pub fn change_path(&mut self, path: PathBuf) {
        self.workdir = path;
    }

    pub fn change_path_str(&mut self, str_path: &str) {
        self.change_path(PathBuf::from(str_path));
    }

    pub fn files_filtered(&self, filter: &[&str], sfl: &mut Vec<String>) -> anyhow::Result<()> {
        /* sfl.clear();
        for file in std::fs::read_dir(&self.workdir)? {
            let file = file?;
            if let Some(filename) = file.file_name().to_str() {
                if filename.ends_with(filter) {
                    sfl.push(filename.to_string());
                }
            }
        }*/

        *sfl = WalkDir::new(&self.workdir)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_string_lossy().to_string())
            .filter(|x| {
                for pattern in filter.iter() {
                    if x.ends_with(*pattern) {
                        return true;
                    }
                }
                false
            })
            .collect();

        Ok(())
    }
    pub fn get_current_path(&self) -> PathBuf {
        return self.workdir.clone();
    }
}

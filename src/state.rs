use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppState {
    last_alive_message: DateTime<Local>,
    logs_dir: PathBuf,
    #[serde(skip)]
    is_changed: bool,
}
impl AppState {
    pub fn is_changed(&self) -> bool {
        self.is_changed
    }

    pub(crate) fn save<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<()> {
        todo!();
        self.is_changed = false;
        Ok(())
    }

    pub(crate) fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        todo!()
    }

    pub(crate) fn new(logs_dir: PathBuf) -> Self {
        Self {
            last_alive_message: Local::now(),
            logs_dir,
            is_changed: Default::default(),
        }
    }
}

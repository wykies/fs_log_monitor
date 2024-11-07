use std::{
    fs,
    io::Write as _,
    path::{Path, PathBuf},
};

use anyhow::Context;
use chrono::{DateTime, Local, NaiveTime};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppState {
    last_alive_msg: DateTime<Local>,
    alive_msg_time: Option<NaiveTime>,
    logs_dir: PathBuf,
    #[serde(skip)]
    is_changed: bool,
}
impl AppState {
    pub fn is_changed(&self) -> bool {
        self.is_changed
    }

    pub(crate) fn save<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<()> {
        let s = ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default())
            .context("failed to convert to ron")?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .with_context(|| {
                format!("failed to open file to save AppState: {:?}", path.as_ref())
            })?;
        file.write_all(s.as_bytes()).with_context(|| {
            format!(
                "failed to write to file to save AppState: {:?}",
                path.as_ref()
            )
        })?;
        self.is_changed = false;
        Ok(())
    }

    pub(crate) fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let s = fs::read_to_string(&path)
            .with_context(|| format!("failed to read file for AppState: {:?}", path.as_ref()))?;
        ron::from_str(&s).with_context(|| {
            format!(
                "failed to deserialize AppState from contents of {:?}",
                path.as_ref(),
            )
        })
    }

    pub(crate) fn new(logs_dir: PathBuf) -> Self {
        Self {
            last_alive_msg: Local::now(),
            // TODO 4: Ensure there is a test to make sure this constant is correct
            alive_msg_time: Some(
                NaiveTime::from_hms_opt(7, 0, 0)
                    .expect("should be valid as it is set at build time"),
            ),
            logs_dir,
            is_changed: Default::default(),
        }
    }
}

use std::path::{Path, PathBuf};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    logs_dir: PathBuf,
    #[serde(skip)]
    is_changed: bool,
}
impl State {
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

    pub(crate) fn new<P: AsRef<Path>>(logs_dir: PathBuf, state_file: P) -> anyhow::Result<Self> {
        Ok(Self {
            logs_dir,
            is_changed: Default::default(),
        })
    }
}

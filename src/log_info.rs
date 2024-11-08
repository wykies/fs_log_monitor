use std::{fmt::Display, path::Path};

use chrono::NaiveDateTime;

pub struct LogInfo {
    pub date_time: NaiveDateTime,
    pub abnormal_outcome: Option<String>,
    pub errors_and_warnings: Vec<String>,
}

impl LogInfo {
    pub fn new<S: AsRef<str>>(file_name: S) -> anyhow::Result<Self> {
        let date_time = todo!();
        let abnormal_outcome = todo!();
        Ok(Self {
            date_time,
            abnormal_outcome,
            errors_and_warnings: Default::default(),
        })
    }

    pub fn extract_errors(&mut self, file_path: &Path) -> anyhow::Result<()> {
        todo!("read log file and extract errors")
    }
}

impl Display for LogInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} errors: {}\n{}\n",
            self.date_time.format("%Y-%m-%d %H:%M:%S"),
            self.errors_and_warnings.len(),
            self.errors_and_warnings.join("\n")
        )
    }
}

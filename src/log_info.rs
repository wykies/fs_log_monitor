use std::{fmt::Display, path::Path, sync::OnceLock};

use anyhow::{bail, Context};
use chrono::NaiveDateTime;
use regex::Regex;

pub struct LogInfo {
    pub date_time: NaiveDateTime,
    pub abnormal_outcome: Option<String>,
    pub errors_and_warnings: Vec<String>,
}

impl LogInfo {
    pub fn new<S: AsRef<str>>(file_name: S) -> anyhow::Result<Self> {
        static CELL_RE: OnceLock<Regex> = OnceLock::new();
        let re = CELL_RE.get_or_init(|| {
            Regex::new(r"(\d\d\d\d-\d\d-\d\d \d\d\d\d\d\d)\.\d\d\d ?(\[.+\])?\.html")
                .expect("failed to compile regex")
        });

        let Some(captures) = re.captures(file_name.as_ref()) else {
            // Assumption: Only log files are present in the log folder
            bail!("regex failed to match filename: {}", file_name.as_ref())
        };
        // Regex matched and can only match if first capture group is found as it is not optional
        let date_time_str = captures.get(1).expect("required for match").as_str();
        let abnormal_outcome = captures.get(2).map(|x| x.as_str().to_string());
        let date_time = NaiveDateTime::parse_from_str(date_time_str, "%F %H%M%S")
            .with_context(|| format!("failed to parse date from {date_time_str:?}"))?;
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
            "{} {} errors: {}\n{}\n",
            self.abnormal_outcome.as_deref().unwrap_or("[ - ]"),
            self.date_time.format("%F %H:%M:%S"),
            self.errors_and_warnings.len(),
            self.errors_and_warnings.join("\n")
        )
    }
}

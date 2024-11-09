use std::path::{Path, PathBuf};

use chrono::Local;
use fs_log_monitor::{process_logs_folder, AppState};

fn samples_folder() -> PathBuf {
    Path::new("tests").join("sample_logs").canonicalize().unwrap()
}

#[test]
fn no_files_pass_filter() {
    let before_app_state_created = Local::now().naive_local();
    let mut actual = AppState::new(samples_folder());
    let expected = actual.clone();
    assert!(
        actual.latest_log_datetime() >= before_app_state_created, 
        "date assumed to be now or later so that the sample logs from the past should not be included"
    );
    
    let logs = process_logs_folder(&mut actual).unwrap();
    assert!(logs.is_empty(), "all samples should be in the past and filtered out");
    
    assert_eq!(actual, expected, "no changes expected to AppState created")
}

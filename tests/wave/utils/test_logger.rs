use wave_engine::wave::utils::logger::*;
use wave_engine::{file_name, function_name, log, trace};

#[test]
fn test_open_log() {
    assert!(init().is_some())
}

#[test]
fn test_reset_logs() {
    let option = init();
    let mut log_file = option.as_ref().unwrap();

    log!(log_file, "INFO", "Testing");

    reset_logs();
    let logs: String = show_logs();
    assert!(!logs.contains("Testing"));
}

#[test]
fn test_show_logs() {
    let option = init();
    let mut log_file = option.as_ref().unwrap();

    log!(log_file, "DEBUG", "Testing");
    let logs: String = show_logs();
    assert!(logs.contains("Testing"));
}

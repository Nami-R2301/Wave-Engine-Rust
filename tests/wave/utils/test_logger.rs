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

pub struct UltraLongStructNameForTesting {}

impl UltraLongStructNameForTesting {
    pub fn long_function_name_for_testing_purposes(_ultra_long_size: u64, _ultra_long_capacity: u64,
                                               _ultra_long_data: UltraLongStructNameForTesting) -> String {
        function_name!()
    }
}

fn long_function_name_for_testing_purposes() -> String {
    function_name!()
}

#[test]
fn test_function_name_length() {
    let function_str: String = UltraLongStructNameForTesting::long_function_name_for_testing_purposes(8, 8,
        UltraLongStructNameForTesting{});
    
    assert_eq!(function_str.len(), 23);
    assert_eq!(function_str, String::from("long_function_name_f..."));
    
    let function_without_namespace = long_function_name_for_testing_purposes();
    
    assert_eq!(function_without_namespace.len(), 23);
    assert_eq!(function_without_namespace, String::from("long_function_name_f..."));
}

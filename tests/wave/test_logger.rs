use app::wave::logger::*;

#[test]
fn test_open_log() {
  assert!(open_log().is_some())
}
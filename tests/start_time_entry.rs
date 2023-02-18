use time_tracker::commands::start_time_entry::start_time_entry;

#[test]
fn it_should_create_time_entry() {
    let entry = start_time_entry("Testa".to_string());
    assert_eq!(entry.name, "Testa");
}

use crate::helpers::get_time_tracker;

mod helpers;

#[test]
fn it_should_create_time_entry() {
    let (commands, queries) = get_time_tracker();
    let test_entry_name = "Merge master".to_string();

    (commands.start_time_entry)(test_entry_name);

    let time_entries = (queries.get_daily_time_entries)();
    let testa_entry = time_entries.iter().find(|e| e.name == test_entry_name);

    assert!(testa_entry.is_some());
}

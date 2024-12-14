use day_13::count_tokens_to_will_all;

#[test]
fn should_count_tokens_to_will_all() {
    assert_eq!(count_tokens_to_will_all("tests/resources/puzzle.txt", None), 480);
}

#[test]
fn should_count_tokens_to_will_all_with_multiplier() {
    assert_eq!(count_tokens_to_will_all("tests/resources/puzzle.txt", Some(10000000000000)), 875318608908);
}
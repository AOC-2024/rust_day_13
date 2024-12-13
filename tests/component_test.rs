use day_13::count_tokens_to_will_all;

#[test]
fn should_count_tokens_to_will_all() {
    assert_eq!(count_tokens_to_will_all("tests/resources/puzzle.txt"), 480);
}
use part2_tests;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, part2_tests::add_two(2));
}
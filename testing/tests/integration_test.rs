use testing::{add_two, add};

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(add_two(3), 5);
}

#[test]
fn it_adds() {
    common::setup();
    assert_eq!(add(2,4), 6);
}


// Test commands

// cargo test --test "file name"; run only the tests in the named integration test file
// Integration Test Commands
// cargo test --test [file-name]

extern crate auto_test;

mod common;

#[test]
fn it_add_two()
{
    common::setup();
    assert_eq!(4, auto_test::add_two(2))
}
extern crate rustlex_codegen as rustlex;
include!(concat!(env!("OUT_DIR"), "/test.rs"));

pub fn main () {
    test_simple();
    test_other();
}

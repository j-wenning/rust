use tests; // doesnt need config declaration, tests dir is reserved for running integration tests, but still needs to import crates

mod common; // in order to not have module run as a test unnecessarily

#[test]
fn it_adds_two() {
  common::setup();
  assert_eq!(4, tests::add_two(2));
}

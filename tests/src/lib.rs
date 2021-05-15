// file is necessary for running integration tests (the intended way)
#[cfg(test)] // guarantees this module will not be included in builds (unit testing only)
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  #[should_panic]
  fn bound_to_fail() {
    panic!("Intended");
  }

  #[test]
  fn larger_can_hold_smaller() {
    assert!(Rect { w: 7, h: 3 }.can_hold(&Rect { w: 3, h: 1 }));
  }

  #[test]
  fn larger_cannot_hold_smaller() {
    assert!(!Rect { w: 3, h: 1 }.can_hold(&Rect { w: 7, h: 3 }));
  }

  #[test]
  fn adds_two() {
    assert_eq!(4, add_two(2));
  }

  #[test]
  fn greet_contains_input() {
    let expected = "Carol";
    let result = greet("Carol");
    assert!(
      result.contains(expected),
      "Greeting did not contain {}, result was {}.",
      expected,
      result
    );
  }

  #[test]
  #[should_panic(expected = "Guess value must be between 1 and 100, received 101.")]
  fn new_guess_should_panic() {
    Guess::new(101);
  }

  #[test]
  fn new_guess() {
    Guess::new(32);
  }

  #[test]
  fn it_also_works() -> Result<(), String> { // cannot use should panic, as errors are wrapped in a result
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("2 + 2 != 4"))
    }
  }

  #[test]
  #[ignore] // will only run test if specified by cargo test args
  fn expensive() { /* totally expensive code here */ }
}

#[derive(PartialEq, Debug)] // partial eq required for assertions, debug required for failed assertions
pub struct Rect {
  pub w: u32,
  pub h: u32,
}

impl Rect {
  pub fn can_hold(&self, rect: &Rect) -> bool {
    self.w > rect.w && self.h > rect.h
  }
}

pub fn add_two(val: i32) -> i32 {
  val + 2
}

pub fn greet(name: &str) -> String {
  format!("Hello {}", name)
}

pub struct Guess {
  val: i32,
}

impl Guess {
  pub fn new(val: i32) -> Guess {
    if val < 1 || val > 100 {
      panic!("Guess value must be between 1 and 100, received {}.", val);
    }
    Guess { val }
  }

  pub fn val(&self) -> i32 {
    self.val
  }
}

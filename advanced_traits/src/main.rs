use std::ops::Add;
use std::fmt;

fn main() {
  let p = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };

  println!("{}", p);

  let mm = Millimeters (10);
  let m = Meters (3);

  println!("{:?}", mm + m);

  let h = Human;

  h.fly(); // defaults to direct implementation
  // Human::fly(&h); // useful for disambiguation
  Pilot::fly(&h);
  Wizard::fly(&h);

  println!("{}", Dog::baby_name()); // method, can infer type
  // println!("{}", Animal::baby_name()); // associated function, cannot infer type
  println!("{}", <Dog as Animal>::baby_name()); // use animal implementation for dog
  // <Type as Trait>::fn(receiver_if_method, next_arg, ...) // no method for associated functions

  p.outline_print();

  let v = vec![String::from("hello"), String::from("world")];

  println!("{:?}", v);

  let w = Wrapper(v);

  println!("{}", w);
}

pub trait Iterator {
  type Item; // removes need to declare generic for next()

  fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

// Add op definition
/*
trait Add<Rhs=Self> {
  type Output;

  fn add(self, rhs: Rhs) -> Self::Output;
}
*/


// operator overload for +
impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

// types would not work for Millimeters -> Meters
#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + other.0 * 1000)
  }
}

trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Pilot for Human {
  fn fly(&self) {
    println!("*Airplane noise*");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("Woosh!");
  }
}

impl Human {
  fn fly(&self) {
    println!("Unfortunate.");
  }
}

trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("puppy")
  }
}

trait OutlinePrint: fmt::Display {
  fn outline_print(&self) {
    let output = self.to_string();
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {} *", output);
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

impl OutlinePrint for Point {}

// required for OutlinePrint
impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

// simple wrapper for Strings, Deref trait may be used for more complex type-handling
struct Wrapper(Vec<String>);

// impl fmt::Display for Vec<String> { // not defined in crate
impl fmt::Display for Wrapper { // bypasses in-crate definition limitation
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0.join(", ")) // self.0 is the tuple-wrapped vector
  }
}

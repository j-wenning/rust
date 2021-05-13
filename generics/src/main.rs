use std::fmt::Display;
use std::any::type_name;

fn main() {
  let largest_num = largest(&[4, 5, 10, 11]);
  let largest_char = largest(&['a', 'z', 'c', 'f']);

  println!("{}, {}", largest_num, largest_char);

  let point = Point { x: 1, y: 2 };
  // let point = Point { x: 1, y: 2.0 }; // type mismatch, x and y must be of same type
  let fpoint = FlexPoint { x: 1, y: 2.0 };

  println!("{}", point.x());

  point.stuff();
  point.more_stuff();

  println!("{:?}", point);
  println!("{:?}", fpoint);

  let fpoint = Point { x: 0.1, y: 0.0 };

  println!("{:?}", fpoint.dist_from_origin());

  ex(&point);

  return_ex().ex();
}

// // if you want to only work with copyable types
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//   let mut largest = list[0];

//   for &item in list {
//     if item > largest {
//       largest = item;
//     }
//   }

//   largest
// }

// properly generic (imo)
fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
    if *item > *largest {
      largest = &item;
    }
  }

  largest
}

#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T,
}

// impl requires generic type definition if type's type is generic
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
  fn own_type(&self) -> String {
    String::from(type_name::<T>())
  }
}

impl<T> Example for Point<T> {}

trait DoStuff {
  fn stuff(&self); // essentially a virtual method
  fn more_stuff(&self) { // an inherited method
    println!("I do stuff!");
    self.stuff();
  }
}

trait Example {
  fn ex(&self) {
    println!("ex");
  }
}

impl<T> DoStuff for Point<T> {
  fn stuff(&self) {
    println!("Point<{}> does stuff!", self.own_type());
  }
}

impl Display for Example { // example of implementing a trait on a trait (trait wrapping/blanket impementation)
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    formatter.align();
    Ok(())
  }
}

impl Point<f32> {
  fn dist_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

#[derive(Debug)]
struct FlexPoint<T1, T2> {
  x: T1,
  y: T2,
}

impl<T1, T2> Example for FlexPoint<T1, T2> {}

// fn ex(arg: &(impl DoStuff + Example)) { // this would work for independently typed params with matching trait(s)
// fn ex<T: DoStuff + Example>(arg: &T) { // this ensures all params of type T are of the same type
fn ex<T>(arg: &T) -> () // same as above, but visibly declutters function signature
  where T: DoStuff + Example, // though splitting it to two lines for one Type is likely unnecessary
{ // this ensures all params of type T are of the same type
  arg.stuff();
  arg.ex();
}

fn return_ex() -> impl Example {
  // if true {
    Point { x: 3, y: 3 } // return types must match
  // } else {
  //   FlexPoint { x: 2, y: 3.0 }
  // }
}

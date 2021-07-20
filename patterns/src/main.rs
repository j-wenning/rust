fn main() {
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  // convenient but not exhaustive compared to match
  if let Some(color) = favorite_color {
    println!("{}.", color);
  } else if is_tuesday {
    println!("Green.");
  } else if let Ok(age) = age { // shadowed age is only valid within block
    if age > 30 {
      println!("Purple.");
    } else {
      println!("Orange.");
    }
  } else {
    println!("Blue.");
  }

  let mut stack = Vec::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  while let Some(top) = stack.pop() {
    println!("{}", top);
  }

  let v = vec!['a', 'b', 'c'];

  for (index, value) in v.iter().enumerate() {
    println!("{}, {}", index, value);
  }

  let _x = 5;

  let (_x, _y, _z) = (1, 2, 3);

  let coords = (25, 32);

  print_coords(&coords);

  let value = Option::Some(5);

  // let Some(x) = value; // cannot bind refutable pattern "Some()", could be "None"

  if let Some(x) = value { // valid becasue of scope isolation on desired value
    println!("{}", x);
  }

  // if let x = 5 { // irrefutable pattern (useless conditional)
  //   println!("{}", x);
  // }

  let x = 3;

  match x { // literal matching
    1 => println!("one"), // literal pattern
    2 | 3 => println!("two or three"), // multiple pattern
    4..=10 => println!("four through ten"), // range pattern
    _ => println!("default"),
  }

  let c = 'a';

  match c { // ranges apply to chars as well
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("other character"),
  }

  let x = Some(4);

  match x {
    Some(4) => println!("four"),
    Some(x) => println!("{}", x), // scoped x, not same as line 66
    None => println!("NaN"),
    // _ => println!("default"), // unreachable
  }

  let p = Point { x: 0, y: 1 };

  match p {
    Point { x: 0, y } => println!("x: 0, {}", y),
    Point { x, y: 0 } => println!("y: 0, {}", x),
    Point { x, y } => println!("Neither 0, {}, {}", x, y),
    // _ => println!("default"), // unreachable
  }

  let msg = Message::ChangeColor(Color::RGB(255, 155, 34));

  match msg {
    Message::Quit => println!("Quitting."),
    Message::Move { x, y } => println!("Moving {}, {}", x, y),
    Message::Write(s) => println!("Writing, {}", s),
    Message::ChangeColor(Color::RGB(r, g, b)) => println!(
      "Changing color, {}, {}, {}",
      r, g, b
    ),
    Message::ChangeColor(c) => println!("Whatever it may be: {:?}", c),
  }

  let Point { x, y } = Point { x: 1, y: 2 };

  foo(x, y);

  let c = Color::RGB(255, 32, 144);

  match c {
    Color::RGB(r, ..) => println!("{}", r),
    _ => (),
  }

  let vals = (1, 2, 3, 4, 5);

  let (first, .., last) = vals;

  println!("{}, {}", first, last);

  let x = Some(5);

  match x { // it is possile to inclue additional conditionals
    Some(x) if x > 4 => println!("Greater than 4"),
    _ => (),
  }

  let p = Point { x: 1, y: 3 };

  match p { // must bind values tested by match (e.g. range values)
    Point { x: x @ 0..=4, y: 3 } => println!("{}", x),
    _ => (),
  }
}

fn print_coords(&(x, y): &(i32, i32)) {
  println!("{}, {}", x, y);
}

struct Point {
  x: i32,
  y: i32,
}

#[derive(Debug)]
enum Color {
  RGB(u8, u8, u8),
  HSV(u8, u8, u8)
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(Color),
}

fn foo(_: i32, y: i32) {
  println!("{}", y);
}

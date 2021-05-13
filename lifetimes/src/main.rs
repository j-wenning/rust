
fn main() {
  let s: &'static str = "a static string slice"; // annotate static lifetime refs as 'static (not necessary for a str slice, as it is inferred)
  let name = Name { first: "fName", last: "lName" };
  println!("{}", s);
  println!("{}", longest("test", "longer"));
  println!("{:?}", name);
  println!("{}", name.announce_and_return_last("announcement"));
  println!("{}", first_word("wow cool"));
}

// this implementation would require the lifetimes of x and y to match
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn first_word(s: &str) -> &str { // valid due to lifetime elision rules (compiler automatically applies lifetimes for this case)
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}

#[derive(Debug)]
struct Name<'a> {
  first: &'a str,
  last: &'a str,
}

impl<'a> Name<'a> {
  fn announce_and_return_last(&self, msg: &str) -> &str {
    println!("{}", msg);
    self.last
  }
}

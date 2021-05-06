fn main() {
  let mut s = String::from("hello");
  let len = calc_len(&s);

  println!("The length of '{}' is {}.", s, len);

  mut_str(&mut s); // must pass in a mutable reference
  println!("{}", s);

  let s1 = &s;
  // let s2 = &mut s; // cannot borrow a mutable ref while an immutable ref has been borrowed
  let s3 = &s; // however, multiple borrows of an immutable ref can occur

  println!("{}, {}", s1, s3);

  let s2 = &mut s; // allowed because s1 and s3 have been moved

  println!("{}", s2);

  let s = proper_factory();

  println!("{}", s);
}

fn calc_len(str: &String) -> usize {
  str.len()
}

fn mut_str(str: &mut String) {
  str.push_str(", world.");
}


// fn improper_factory() -> &String { // returns a ref that points to deallocated memory
//   let s = String::from("hello");
//   &s
// }

fn proper_factory() -> String {
  let s = String::from("hello");
  s
}

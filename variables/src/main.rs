fn main() {
  let x = 5;
  print(&x);
  let mut x = 6;
  print(&x);
  x += 1;
  print(&x);
}

fn print(&val: &u32) {
  println!("The value of x is {}.", val);
}

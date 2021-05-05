fn main() {
  let five = five();
  another_function(&five, &inc(&five));
}

fn another_function(&x: &i32, &y: &i32) {
  print(&'x', &x);
  print(&'y', &y);
}

fn five() -> i32 {
  5
}

fn inc(&val: &i32) -> i32 {
  val + 1
}

fn print(&name: &char, &val: &i32) {
  println!("The value of {} is {}.", name, val)
}

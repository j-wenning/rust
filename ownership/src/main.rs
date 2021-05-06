fn main() {
  let s = "hello";

  // s += ", world."; // str is immutable

  let mut s = String::from(s);

  s.push_str(", world.");
  println!("{}", s);

  let mut s1 = s;

  // println!("{}", s); // s was moved to s1
  s = s1.clone(); // deep copy
  println!("{}, {}", s1, s);
  owner(s);
  // println!("{}", s); // s was moved into owner

  let a = 1;
  let b = a;

  println!("{}, {}", a, b); // value was copied
  copier(a);
  println!("{}", a); // value was copied

  s1 = owner_returner(s1);
  println!("{}", s1); // s1 was moved into owner_returner and then moved to return value of owner_returner
}

fn owner(str: String) {
  println!("{}", str);
}

fn copier(num: u32) {
  println!("{}", num);
}

fn owner_returner(str: String) -> String {
  println!("{}", str);
  str
}

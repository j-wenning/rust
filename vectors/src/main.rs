fn main() {
  let mut v = vec![0, 3, 9];

  println!("{:?}", v);
  v.push(17);
  println!("{:?}", v);
  // println!("{}", v[100]); // crashes
  println!("{:?}", v.get(100)); // handles gracefully

  let v = vec![Test {}, Test {}];
  // let v0 = v[0]; // invalid, v[0] cannot be copied
  let v0 = &v[0];

  println!("{:?}", v0);

  let mut v = vec![0, 1, 2, 3];

  for i in &mut v {
    *i += 10;
  }

  println!("{:?}", v);

  let mut v: Vec<Types> = Vec::new();

  v.push(Types::TypeA(1));
  v.push(Types::TypeB(String::from("aaaaaa")));
  println!("{:?}", v);
  v.pop();
  println!("{:?}", v);
}

#[derive(Debug)]
struct Test {}

#[derive(Debug)]
enum Types {
  TypeA(i32),
  TypeB(String)
}

fn main() {
  let a = call_add_one(add_one, 1);

  println!("{}", a);

  // tuple initializers are implemented as functions that return an instance based on supplied params
  let statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

  println!("{:?}", statuses);

  let v = return_closure()(1);

  println!("{}", v);

  let v = return_fn()(1);

  println!("{}", v);
}

fn add_one(x: i32) -> i32 {
  x + 1
}

fn call_add_one(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg)
}

#[derive(Debug)]
enum Status {
  Value(u32),
  _Stop,
}

// fn size is unknown at compile time
// fn return_closure() -> dyn Fn(i32) -> i32 {
//   |x| x + 1
// }

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

fn return_fn() -> fn(i32) -> i32 {
  add_one
}

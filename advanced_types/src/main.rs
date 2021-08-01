fn main() {
  let x = 5;
  let y: Kilometers = 5;

  println!("{}", x + y);

  let f: Thunk = Box::new(|| println!("test"));

  takes_long_type(f);

  let s = "testing";

  returns_long_type(s)();

  println!("{}", s);

  let v = do_panic(false);

  println!("{}", v);

  never_loop();
}

type Kilometers = i32;

// type alias can reduce repetition
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
  f();
}

fn returns_long_type(s: &'static str) -> Thunk {
  Box::new(move || println!("{}", s))
}

// type alias can regulate consistency
type _Result<T> = Result<T, dyn std::error::Error>;

// ! represents never type, which may be coerced into any type
fn _will_panic() -> ! {
  panic!("panic");
}

fn do_panic(b: bool) -> u32 {
  match b {
    true => panic!("panic"),
    false => 100,
  }
}

fn never_loop() {
  let mut input = String::new();

  loop {
    std::io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let guess: u32 = match input.trim().parse() {
      Ok(s) => s,
      Err(_) => {
        input.clear();
        continue // continue is of type never
      },
    };

    println!("{}", guess);

    break;
  }
}

// allegedly there is some difference here beteween Sized and ?Sized
// nothing I've found has seemed to justify/explain this distinction
// fn _sized_generic<T: Sized + std::fmt::Debug>(t: T) {
fn _sized_generic<T: std::fmt::Debug>(t: T) { // shorthand form, Sized is assumed
  println!("{:?}", t);
}

fn _unsized_generic<T: ?Sized + std::fmt::Debug>(t: &T) {
  println!("{:?}", t);
}

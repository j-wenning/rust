use std::{ fs::{ self, File }, io::{ self, Read, ErrorKind } };

fn main() {
  // panic!("aaaaaaaaa"); // essentially a top-level throw

  // let val = [1, 2, 3][99]; // index out of bounds panic
  let filename = "hello.txt";
  let f = File::open(filename);
  // let f = f.unwrap(); // will always panic on error
  // let f = f.expect(&format!("Failed to open file: {}", filename)); // custom panic message
  let f = f.unwrap_or_else(|err| { // unwrap consumes self
    if err.kind() == ErrorKind::NotFound {
      File::create(filename).unwrap_or_else(|err| {
        panic!("Problem creating file: {:?}", err);
      })
    } else {
      panic!("Problem opening file: {:?}", err);
    }
  });

  println!("{:?}", f);

  let result = read_file(&filename);

  println!("{:?}", result);

  // let f = File::open(filename)?; // "?" can only be used in a function that implements std::ops::Try (e.g. Result, Option)

  // let guess = Guess::new(101);
  let guess = Guess::new(32);

  println!("{:?}", guess)
}

#[derive(Debug)]
pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, received {}.", value);
    }
    Guess { value }
  }
  pub fn value(&self) -> i32 { // only necessary when implemented in a manner that utilizes public declarations
    self.value
  }
}

// // implementation of main that would allow for top-level "?"
// use std::{ error::Error, fs::File };

// fn main() -> Result<(), Box(dyn Error)> {
//   let filename = "hello.txt";
//   let f = File::open(filename)?;
//   Ok(())
// }

fn read_file(s: &str) -> Result<String, io::Error> {
  // // longform version
  // let f = File::open(s);

  // let mut f = match f {
  //   Ok(file) => file,
  //   Err(e) => return Err(e)
  // };

  // let mut buf = String::new();

  // match f.read_to_string(&mut buf) {
  //   Ok(_) => Ok(buf),
  //   Err(e) => Err(e)
  // }

  // // shorthand version
  // let mut f = File::open(s)?;
  // let mut buf = String::new();
  // f.read_to_string(&mut buf)?;
  // Ok(s)

  // // we're golfing now
  // let mut buf = String::new();
  // File::open(s)?.read_to_string(&mut buf)?;
  // Ok(buf)

  // this is so common that its already done for us anyways
  fs::read_to_string(s)
}

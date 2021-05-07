fn main() {
  let mut user = User {
    online: false,
    sign_ins: 3,
    name: String::from("aUserName"),
    email: String::from("example@test.co"),
  };

  println!("{:#?}", user);
  user.sign_in();
  println!("{:#?}", user);
  user.name = String::from("aNewName");
  println!("{:#?}", user);
  user.sign_out();
  println!("{:#?}", user);

  let User { name, email, sign_ins, online } = User::build("email@mail.co", "aDifferentUsername");

  println!("{}\n{}\n{}\n{}", name, email, sign_ins, online);

  let User { name, email, sign_ins, online } = User {
    email: String::from("test@test.co"),
    name: String::from("aTest"),
    ..user
  };

  println!("{}\n{}\n{}\n{}", name, email, sign_ins, online);

  let color_tup = Color(255, 0, 255);
  let Color(r, g, b) = color_tup;
  let point_tup = Point(0, 0 , 255);
  // let color_as_point_tup: Point = Color(0, 0, 7); // type mismatch

  println!("{}, {}, {}", color_tup.0, color_tup.1, color_tup.2);
  println!("{}, {}, {}", r, g, b);
  println!("{:?}", point_tup);
}

#[derive(Debug)]
struct User {
  name: String,
  email: String,
  sign_ins: u64,
  online: bool
}

impl User {
  fn sign_in(&mut self) {
    self.online = true;
    self.sign_ins += 1;
  }

  fn sign_out(&mut self) {
    self.online = false;
  }

  fn build(email: &str, name: &str) -> User {
    User {
      name: String::from(name),
      email: String::from(email),
      sign_ins: 1,
      online: true,
    }
  }
}

struct Color(u8, u8, u8);

#[derive(Debug)]
struct Point(u8, u8, u8);

fn main() {
  let ipv4 = IP::V4(127, 0, 0, 1);
  let ipv6 = IP::V6(String::from("::1"));

  route(ipv4);
  route(ipv6);

  let num = 1;
  let mut opt_num: Option<u32> = None;

  // println!("{}", num + opt_num); // invalid

  println!("{}", num + opt_num.unwrap_or(0));
  opt_num = Some(1);
  println!("{}", num + opt_num.unwrap_or(0));

  if let Some(3) = opt_num {
    println!("opt_num is 3");
  } else {
    println!("opt_num isn't 3")
  }

  opt_num = Some(3);

  if let Some(3) = opt_num {
    println!("opt_num is 3");
  }
}

#[derive(Debug)]
enum IP {
  V4(u8, u8, u8, u8),
  V6(String),
}

fn route(ip: IP) {
  println!("{:?}", ip);
  match ip {
    IP::V4(_, _, _, _) => println!("Only 32 bits?  Lame."),
    IP::V6(_) => println!("Nice."),
  }
}

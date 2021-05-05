fn main() {
  let num = 3;

  if num < 5 {
    println!("Condition is true.");
  } else {
    println!("Condition is false.");
  }

  println!("Condition is {}.", num < 5);

  println!("Condition is {}.", if true { "true" } else { "false" });

  let mut ct = 0;

  let result = loop {
    ct += 1;
    if ct >= 10 {
      break ct
    }
  };

  println!("Result is {}.", result);

  let mut i = 0;

  while i < 3 {
    println!("Index is {}.", i);
    i += 1;
  }

  let arr = [4, 5, 3, 1];

  for val in arr.iter() {
    println!("Value is {}.", val);
  }

  for val in (1..4).rev() {
    println!("Value is {}.", val);
  }
}

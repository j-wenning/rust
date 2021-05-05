use rand::Rng;

fn main() {
  let min = 0;
  let max = 100;
  let secret_num = rand::thread_rng().gen_range(min, max);

  let mut guesses = 0;

  loop {
    println!("Pick a number between {} and {}:", min, max - 1);

    let mut guess = String::new();

    std::io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line.");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue
    };

    guesses += 1;

    match guess.cmp(&secret_num) {
      std::cmp::Ordering::Less => println!("Too low."),
      std::cmp::Ordering::Greater => println!("Too high."),
      std::cmp::Ordering::Equal => {
        println!("Correct.  Took {} guesses.", guesses);
        break;
      },
    };
  }
}

use mini_grep::{ Config, run };

fn main() {
  let config = Config::new(std::env::args()).unwrap_or_else(|err| {
    eprintln!("Error parsing arguments: {}", err);
    std::process::exit(1);
  });

  if let Err(e) = run(config) {
    eprintln!("Error: {}", e);
    std::process::exit(1);
  }
}

pub struct Config {
  pub query: String,
  pub path: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(v) => v,
      None => return Err("Missing query parameter."),
    };
    let path = match args.next() {
      Some(v) => v,
      None => return Err("Missing file parameter."),
    };
    let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { query, path, case_sensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
  let Config { query, path, case_sensitive } = config;
  let contents = std::fs::read_to_string(path)?;

  let results = if case_sensitive {
    search(&query, &contents)
  } else {
    search_case_insensitive(&query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = &query.to_lowercase();
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(query))
    .collect()
}

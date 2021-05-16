#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "dUcT";
    let contents = "\
      Rust:\n\
      safe, fast, productive.\n\
      Pick three.\
    ";
    let expected: Vec<String> = Vec::new();

    assert_eq!(search(query, contents), expected);
  }

  #[test]
  fn case_insensitive() {
    let query = "dUcT";
    let contents = "\
      Rust:\n\
      safe, fast, productive.\n\
      Pick three.\
    ";
    let expected = vec!["safe, fast, productive."];

    assert_eq!(search_case_insensitive(query, contents), expected);
  }
}

pub struct Config<'a> {
  pub query: &'a str,
  pub path: &'a str,
  pub case_sensitive: bool,
}

impl<'a> Config<'a> {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    let query = args.get(1);
    if query.is_none() { return Err("Missing query parameter."); }
    let query = query.unwrap();

    let path = args.get(2);
    if path.is_none() { return Err("Missing file parameter."); }
    let path =  path.unwrap();

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
  let mut v = Vec::new();
  for line in contents.lines() {
    if line.contains(query) { v.push(line); }
  }
  v
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = &query.to_lowercase();
  let mut v = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(query) { v.push(line); }
  }
  v
}

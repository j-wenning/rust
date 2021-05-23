use mini_grep::*;

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

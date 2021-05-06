fn main() {
  let /* mut */ str = String::from("hello, world.");
  let str_first_word_len = first_word_len(&str);

  println!("{}", str_first_word_len);

  let result = first_word(&str);

  // str.clear();  // attempt to perform a mutable borrow after an immutable borrow

  println!("The first word of '{}' is '{}'.", str, result);

  let str = &String::from("hello, world.");
  let result = first_word(str); // works on strings and string literals (slices)

  println!("The first word of '{}' is '{}'.", str, result);
}

fn first_word_len(str: &String) -> usize {
  let bytes = str.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  str.len()
}

fn first_word(str: &String) -> &str {
  let len = first_word_len(&str);
  &str[..len]
}

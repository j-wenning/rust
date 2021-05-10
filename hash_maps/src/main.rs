#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use regex::Regex;

fn main() {
  let mut scores = HashMap::new();

  scores.insert("red", 10);
  scores.insert("blue", 20);
  println!("{:?}", scores);

  let teams = vec!["red", "blue"];
  let scores = vec![10, 20];
  // zip teams iter and scores iter and collect
  // inferred hash map key/val types
  let scores: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

  println!("{:?}", scores);

  let key = String::from("color");
  let val = String::from("red");
  let mut map = HashMap::new();

  map.insert(key, val);
  println!("{:?}", map);
  // println!("{}, {}", key, val); // invalid, key and val were moved
  println!("{:?}", map.get("color"));

  let key = "color";

  println!("{:?}", map.get(key));
  for (key, value) in scores {
    println!("{}, {}", key, value);
  }
  map.insert(String::from("color"), String::from("blue"));
  println!("{:?}", map);
  map.entry(String::from("color")).or_insert(String::from("red"));
  map.entry(String::from("flavor")).or_insert(String::from("sweet"));
  println!("{:?}", map);

  assert!(!anagram("yes", "no"));
  assert!(anagram("A decimal point", "I’m a dot in place."));
  assert!(anagram("baaa", "aaba"));
  assert!(anagram("Dormitory", "Dirty room."));
  assert!(!anagram("aaaa", "bbbb"));
  assert!(anagram("Funeral", "Real fun."));
}


fn anagram(s1: &str, s2: &str) -> bool {
  let mut map1: HashMap<char, u32> = HashMap::new();
  let mut map2: HashMap<char, u32> = HashMap::new();
  lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)[^a-z]").unwrap();
  }
  let s1 = &RE.replace_all(&s1, "").to_lowercase();
  let s2 = &RE.replace_all(&s2, "").to_lowercase();

  count_chars(s1, &mut map1);
  count_chars(s2, &mut map2);
  for (key, val) in map1 {
    if &val != map2.get(&key).unwrap_or(&0) { return false; }
  }
  true
}

fn count_chars(s: &str, map: &mut HashMap<char, u32>) {
  for i in s.chars() {
    let count = map.entry(i).or_insert(0);
    *count += 1;
  }
}

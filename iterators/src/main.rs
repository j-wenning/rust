fn main() {
  let v = vec![0, 3, 7];
  let v_it = v.iter();

  for val in v_it {
      println!("{}", val);
  }

  let v_it = v.iter();
  let sum: i32 = v_it.sum(); // sum is a consuming adaptor

  // println!("{}", v_it.next().unwrap()); // v_it was moved
  println!("{}", sum);

  let v_it = v.iter();
  let map = v_it.map(|x| x - 1);

  println!("{:?}", map);

  let collected: Vec<_> = map.collect(); // must be called to access the altered vals

  println!("{:?}, {:?}", v, collected);
}

struct BoundedCounter {
  count: u32,
}

impl BoundedCounter {
  fn new() -> BoundedCounter {
    BoundedCounter { count: 0 }
  }
}

impl Iterator for BoundedCounter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      return Some(self.count)
    }
    None
  }
}

#[derive(PartialEq, Debug)]
struct Shoe<'a> {
  size: u32,
  style: &'a str,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  shoes
    .into_iter() // consumed shoes
    .filter(|Shoe { style: _, size }| size == &shoe_size)
    .collect()
}

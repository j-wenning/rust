#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn caches_multiple() {
    let mut cacher = Cacher::new(|v| v);

    let val = &3;
    assert_eq!(val, cacher.value(val));

    let val = &2;
    assert_eq!(val, cacher.value(val));
  }

  #[test]
  fn caches_multiple_types() {
    let mut cacher = Cacher::new(|v| v);
    let val = &"test";
    assert_eq!(val, cacher.value(val));

    let mut cacher = Cacher::new(|v| v);
    let val = &true;
    assert_eq!(val, cacher.value(val));

    let mut cacher = Cacher::new(|v| v);
    let val = &25;
    assert_eq!(val, cacher.value(val));
  }

  #[test]
  fn is_caching() {
    let mut count = 0;
    let mut cacher = Cacher::new(|v| {
      count += 1;
      v
    });

    let val = &5;
    cacher.value(val);
    cacher.value(val);
    cacher.value(val);

    assert_eq!(1, count, "Failed to cache value.");
    // using cacher beyond this point will produce a borrowing error, count is still captured by the closure
  }
}

use std::{ hash::Hash, collections::HashMap };

pub fn generate_workout<'a>(intensity: &'a u32, rand_num: u32) {
  let mut expensive_result = Cacher::new(|num| {
    println!("Calculating...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    num
  });

  if *intensity < 25 {
    println!(
      "Today, do {} pushups and {} situps.",
      expensive_result.value(intensity),
      expensive_result.value(intensity),
    );
  } else if rand_num == 3 {
    println!("Today, take a break.");
  } else {
    println!("Today, run for {} minutes.", expensive_result.value(intensity))
  }
}

pub struct Cacher<'a, T1, T2, T3>
where T1: FnMut(&T2) -> &T3,
      T2: Eq + Hash,
      T3: Copy,
{
  calculation: T1,
  values: HashMap<&'a T2, &'a T3>,
}

impl<'a, T1, T2, T3> Cacher<'a, T1, T2, T3>
where T1: FnMut(&T2) -> &T3,
      T2: Eq + Hash,
      T3: Copy,
{
  pub fn new(calculation: T1) -> Cacher<'a, T1, T2, T3> {
    Cacher { calculation, values: HashMap::new() }
  }

  pub fn value(&mut self, arg: &'a T2) -> &'a T3 {
    let calculation = &mut self.calculation;
    self.values.entry(arg).or_insert_with(|| calculation(arg))
  }
}

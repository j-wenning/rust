#[macro_export]
macro_rules! vec_simple {
  // only matched expressions allowed
  // $() denotes capture group
  // $x denotes value within group, specified to be an expr(ession)
  // * denotes 0+ matches of preceding values
  ( $( $x:expr),* ) => {
    {
      let mut temp_vec = Vec::new();
      $(
        temp_vec.push($x);
      )*
      temp_vec
    }
  };
}

fn main() {
  let v = vec_simple!(1, 23, 50);

  println!("{:?}", v);

  let v = vec_simple![1, 23, 50];

  println!("{:?}", v);
}

// TODO: https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes

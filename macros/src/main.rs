use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

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

  Pancakes::hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;

// procedural macro derivatives

// // attribute-like macro
// // #[route(GET, "/")]
// // fn index() {}
//
// // additional parameter to handle the body of the attached item (i.e. fn index(){})
// // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
//
// // function-like macro
// // let sql = sql!(SELECT * FROM posts WHERE id = 1);
//
// // similar to declarative macro with additional capability of including syntax validation
// // #[proc_macro]
// // pub fn sql(input -> TokenStream) -> TokenStream {}

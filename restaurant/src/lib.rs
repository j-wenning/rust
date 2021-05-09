mod foh {
  pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
  }

  pub mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}

    pub mod boh {
      pub struct Breakfast {
        pub main_course: String,
        seasonal_fruit: String,
      }

      pub enum Meal {
        Breakfast(Breakfast),
        Lunch,
        Dinner,
      }

      impl Breakfast {
        pub fn make(main_course: &str) -> Breakfast {
          Breakfast {
            main_course: String::from(main_course),
            seasonal_fruit: String::from("strawberry")
          }
        }
        pub fn read(&self) {
          println!("{}, {}", self.main_course, self.seasonal_fruit);
        }
      }

      pub fn cook_order(meal: &Meal) {
        if let Meal::Breakfast(breakfast) = meal {
          println!("The main course is {}.  The seasonal fruit is {}.", breakfast.main_course, breakfast.seasonal_fruit);
        }
      }
      pub fn fix_order(new_meal: &Meal) {
        cook_order(&new_meal);
        super::serve_order();
      }
    }
  }
}

pub use self::foh::serving::boh as back_of_house;

use std::{io, cmp::Ordering, collections::*};

pub fn eat_at_restaurant() {
  // crate::foh::hosting::add_to_waitlist(); // 'absolute' path

  foh::hosting::add_to_waitlist(); // relative path
}

mod ext;

use crate::ext::ext_mod::example as ex;


pub fn example() {
  // let breakfast = foh::serving::boh::Breakfast {
  //   main_course: String::from("toast"),
  //   seasonal_fruit: String::from("strawberry") // can't access, is private
  // };

  let breakfast = back_of_house::Breakfast::make("toast");
  ex();
}

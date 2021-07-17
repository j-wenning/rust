use crate::List::{Cons, Nil};
use crate::BadList::{Cons as BadCons, Nil as BadNil};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

fn main() {
  let b = Box::new(5);

  println!("{}", b);

  // let list = Cons(1,
  //   Box::new(Cons(2,
  //     Box::new(Cons(3,
  //       Box::new(Nil))))));

  // println!("{:?}", list);

  let a = 5;
  let b = Box::new(a);

  assert_eq!(5, *b);

  let a = 3;
  let b = MyBox::new(a);

  assert_eq!(3, *b);

  let me = MyBox::new(String::from("Me"));

  hello(&me);

  let _a = CustomSmartPointer{ data: "some stuff" };
  let b = CustomSmartPointer{ data: "more stuff" };

  println!("Created CustomSmartPointers.");

  drop(b);

  // let a = Rc::new(Cons(5,
  //   Rc::new(Cons(6,
  //     Rc::new(Nil)))));

  // println!("{}", Rc::strong_count(&a));

  // let _b = Cons(7, Rc::clone(&a));

  // println!("{}", Rc::strong_count(&a));
  // {
  //   let _c = Cons(8, Rc::clone(&a));

  //   println!("{}", Rc::strong_count(&a));
  // }

  // println!("{}", Rc::strong_count(&a));

  // let a = 5;
  // let b = &mut a;

  let val = Rc::new(RefCell::new(5));

  let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
  let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

  *val.borrow_mut() += 10;

  println!("{:?}\n{:?}\n{:?}", a, b, c);

  let a = Rc::new(BadCons(5, RefCell::new(Rc::new(BadNil))));

  println!("{}", Rc::strong_count(&a));
  println!("{:?}", a.tail());

  let b = Rc::new(BadCons(10, RefCell::new(Rc::clone(&a))));

  println!("{}", Rc::strong_count(&a));
  println!("{}", Rc::strong_count(&b));
  println!("{:?}", b.tail());

  if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
  }

  println!("{}", Rc::strong_count(&a));
  println!("{}", Rc::strong_count(&b));

  // println!("{:?}", a.tail()); // stack overflow

  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  // println!("{:?}", leaf.parent.borrow().upgrade());

  println!(
    "{}, {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)
  );

  {
    let branch = Rc::new(Node {
      value: 5,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
      "{}, {}",
      Rc::strong_count(&branch),
      Rc::weak_count(&branch)
    );

    println!(
      "{}, {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf)
    );
  }

  println!("{:?}", leaf.parent.borrow().upgrade());
  println!(
    "{}, {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)
  );
}

#[derive(Debug)]
enum List {
    // Cons(i32, List), // infinitely sized
    // Cons(i32, Box<List>), // similar to holding a ref
    // Cons(i32, Rc<List>), // allows for multiple owners
    Cons(Rc<RefCell<i32>>, Rc<List>), // additional inner mutability
    Nil,
}

#[derive(Debug)]
enum BadList {
  Cons(i32, RefCell<Rc<BadList>>),
  Nil,
}

impl BadList {
  fn tail(&self) -> Option<&RefCell<Rc<BadList>>> {
    match self {
      BadCons(_, item) => Some(item),
      BadNil => None,
    }
  }
}

struct MyBox <T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

struct CustomSmartPointer<'a> {
  data: &'a str,
}

impl<'a> Drop for CustomSmartPointer<'a> {
  fn drop(&mut self) {
    println!("Dropping CustomSmartPointer with data \"{}\"!", self.data);
  }
}

pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where T: Messenger, {
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("Rate limit has been reached.");
    } else if percentage_of_max >= 0.9 {
      self.messenger.send("90% of rate limit has been reached.");
    } else if percentage_of_max >= 0.75 {
      self.messenger.send("75% of rate limit has been reached.");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn sends_75_percent_warning() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}

#[derive(Debug)]
struct Node {
  value: i32,
  parent: RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>,
}

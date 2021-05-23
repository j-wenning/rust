use iterators::*;

mod tests {
  #[test]
  fn counter_next() {
    let mut counter = BoundedCounter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
  }

  #[test]
  fn filters_by_size() {
    let shoes = vec![
      Shoe {
        size: 10,
        style: "sneaker",
      },
      Shoe {
        size: 13,
        style: "sandal",
      },
      Shoe {
        size: 10,
        style: "boot",
      },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
      in_my_size,
      vec![
        Shoe {
          size: 10,
          style: "sneaker"
        },
        Shoe {
          size: 10,
          style: "boot"
        },
      ]
    );
  }

  #[test]
  fn it_demo() {
    let v = vec![0, 3, 7];
    let mut v_it = v.iter();

    assert_eq!(v_it.next(), Some(&0));
    assert_eq!(v_it.next(), Some(&3));
    assert_eq!(v_it.next(), Some(&7));
    assert_eq!(v_it.next(), None);
  }
}
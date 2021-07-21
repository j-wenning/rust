use std::slice;

extern "C" {
  fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Called a Rust function from C!")
}

static mut COUNTER: i32 = 0;

fn main() {
  let mut num = 5;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  println!("{:?}, {:?}", r1, r2);
  // println!("{:?}, {:?}", *r1, *r2); // unsafe
  unsafe {
    println!("{:?}, {:?}", *r1, *r2);
  }


  let address = 0x12345usize;
  let r = address as *const i32;

  println!("{:?}", r);

  // dangerous(); // unsafe
  unsafe {
    dangerous();
  }

  let slice = &mut [1, 2, 3, 4];

  let tuple = split_as_mut(slice, 3);

  println!("{:?}", tuple);

  unsafe {
    println!("{}", abs(-7));
  }

  // // unsafe due to potential read/write conflicts for multi-threaded ops
  // COUNTER += 1;
  // println!("{}", COUNTER);

  unsafe {
    COUNTER += 1;
    println!("{}", COUNTER);
  }

  let u: U = U { f: 3.5 };

  // println!("{}", u.i); // unsafe
  unsafe {
    println!("{}", u.f);
    println!("{}", u.i);
  }

  // let f = u.f; // unsafe
  let f = unsafe { u.f };

  println!("{}", f);
}

unsafe fn dangerous() {}

// safe fn wrapping unsafe block
fn split_as_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();
  let ptr = slice.as_mut_ptr();

  assert!(mid <= len);

  unsafe {
    (
      slice::from_raw_parts_mut(ptr, mid),
      slice::from_raw_parts_mut(ptr.add(mid), len - mid)
    )
  }
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}

union U {
  i: i32,
  f: f32,
}

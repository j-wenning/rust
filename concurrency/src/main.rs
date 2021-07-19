use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("spawned thread {}", i);
      thread::sleep(Duration::from_millis(1));
    }
  });
  for i in 1..5 {
    println!("main thread {}", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();

  let v = vec![1, 2, 3];

  thread::spawn(move || {
    println!("{:?}", v);
  });

  // drop(v); // v moved to thread

  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
    // println!("{:?}", val); // val moved to transmitter
  });

  let received = rx.recv().unwrap();
  println!("{}", received);

  let (tx, rx) = mpsc::channel();

  let tx1 = tx.clone();
  thread::spawn(move || {
    let v = vec!["test", "from", "thread"];

    for str in v {
      tx.send(str).unwrap();
      thread::sleep(Duration::from_millis(1));
    }
  });

  thread::spawn(move || {
    let v = vec!["more", "messages"];

    for str in v {
      tx1.send(str).unwrap();
      thread::sleep(Duration::from_millis(1));
    }
  });

  for received in rx {
    println!("{}", received);
  }

  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6;
  }

  println!("{:?}", m);

  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  println!("{}", *counter.lock().unwrap());
}

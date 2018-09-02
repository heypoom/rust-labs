use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn times(x: u32) -> impl Fn(u32) -> u32 {
  move |y| x * y
}

fn main() {
  let (tx, rx) = mpsc::channel();

  for i in 1..10 {
    let txm = tx.clone();

    let h = thread::spawn(move || {
      txm.send(i).unwrap();
    });

    h.join().unwrap();
  }

  let sum = |x, y| x + y;

  let vx = rx.try_iter().map(times(4)).fold(0, sum);

  println!("{:?}", vx);
}

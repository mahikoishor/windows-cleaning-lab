use std::{thread, time::Duration};

pub fn sleep(mills: u64) {
  thread::sleep(Duration::from_millis(mills));
}
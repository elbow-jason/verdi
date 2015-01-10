use std::time::duration::{Duration};
use std::thread::Thread;

fn main() {
  tick_per_second();
}

fn tick_per_second(){
  let mut clock = Clock{interval: Duration::nanoseconds(1), count: 0};
  clock.tick()
}

struct Clock {
  interval: Duration,
  count: u64,
}

impl Clock {
  fn tick(&mut self){
    loop {
      self.count = self.count + 1;
      println!("tick {}", self.count);
      std::io::timer::sleep(self.interval);
    }
  }
}


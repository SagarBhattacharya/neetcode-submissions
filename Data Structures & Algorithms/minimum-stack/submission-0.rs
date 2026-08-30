struct MinStack {
  data: Vec<(i32, i32)>,
}

impl MinStack {
  pub fn new() -> Self {
    Self { data: Vec::new() }
  }

  pub fn push(&mut self, val: i32) {
    let min = match self.data.last() {
      Some(&(_, min)) => val.min(min),
      None => val,
    };
    self.data.push((val, min));
  }

  pub fn pop(&mut self) {
    self.data.pop();
  }

  pub fn top(&self) -> i32 {
    self.data.last().copied().unwrap().0
  }

  pub fn get_min(&self) -> i32 {
    self.data.last().copied().unwrap().1
  }
}
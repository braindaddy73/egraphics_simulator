pub struct History {
  history: [f32, 100],
}

impl Data {

  pub fn clear() {
    for i in 0..99 {
      history[i] = 0;
  }

  pub fn update(&mut self, val: f32) {
    shift_left();
    history[99] = val;
  }
    
  fn shift_left (&mut self) {
    for i in 1..99 {
      history[i-1] = history [i];
  }
    
}

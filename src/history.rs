#[derive(Default)]
pub struct History {
  history: [f32, 100],
  mean: f32,
}

impl History {

  pub fn new() -> Self {
    Self {
      mean : 0.0,
      history : [0.0; 100] 
    }
  }

  pub fn update(&mut self, val: f32) {
    shift_left();
    history[99] = val;
    update_mean();
  }
    
  fn shift_left (&mut self) {
    for i in 1..99 {
      history[i-1] = history [i];
    }
  }
    
  fn update_mean (&mut self) {
    buf = 0;
    for i in 0.99 {
      buf += history[i];
    }
    mean = buf / 100.00;
  }

}

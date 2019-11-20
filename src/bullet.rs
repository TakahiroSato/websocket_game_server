use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Bullet {
  pub x: i32,
  pub y: i32,
  pub mx: i32,
  pub my: i32,
}

impl Bullet {
  pub fn set_position(&mut self) {
    self.x += self.mx;
    self.y += self.my;
  }
}
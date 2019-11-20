use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
  pub id: String,
  pub x: i32,
  pub y: i32,
  pub degree: i32,
  pub life: i32,
}

impl User {
  pub fn set_data(&mut self, x: i32, y: i32, degree: i32) {
    self.x = x;
    self.y = y;
    self.degree = degree;
  }

  pub fn add(&mut self, x: i32, y: i32, degree: i32) {
    self.x += x;
    self.y += y;
    self.degree += degree;
  }

  pub fn get_json(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }
}
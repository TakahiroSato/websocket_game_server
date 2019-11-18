use crate::json_parse;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
  pub id: String,
  pub x: i32,
  pub y: i32,
  pub degree: i32,
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

#[derive(Serialize, Deserialize)]
pub struct Users {
  pub list: Vec<User>,
}

impl Users {
  pub fn add_user(&mut self, id: &String, x: i32, y: i32, degree: i32) {
    self.list.push(User {
      id: id.to_string(),
      x: x,
      y: y,
      degree: degree,
    });
  }

  pub fn remove_user(&mut self, id: &String) {
    self.list = self
      .list
      .iter()
      .filter(|&user| user.id != id.to_string())
      .cloned()
      .collect();
  }

  pub fn get_json(&self) -> String {
    let ret = serde_json::to_string(&self).unwrap();
    ret
  }
}

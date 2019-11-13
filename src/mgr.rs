use crate::json_parse;

//use std::cell::{Cell, RefCell};
use serde_json::{Value, json};
use uuid::Uuid;

pub struct User {
  pub id: String,
  pub data: Value
}

impl User {
  pub fn set_data(&mut self, x: i32, y: i32) {
    self.data["x"] = json!(x);
    self.data["y"] = json!(y);
  }
}

pub struct Users {
  pub list: Vec<User>
}

impl Users {
  pub fn add_user(&mut self, id: &String) {
    self.list.push(User {
      id: id.to_string(),
      data: json_parse::parse(r#"{"x":0, "y":0}"#.to_string()).unwrap()
    });
  }
}
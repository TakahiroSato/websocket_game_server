use serde_json::{Result, Value};

pub fn parse(msg: String) -> Result<Value> {
  println!("{:?}", &msg);
  let v: Value = serde_json::from_str(&msg)?;
  println!("{:?}", v);
  Ok(v)
}
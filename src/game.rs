use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::f32;
use std::time::Instant;
use ws::{Message, Sender};
use std::sync::Mutex;

use crate::bullet::Bullet;
use crate::user::User;

#[derive(Serialize, Deserialize)]
pub struct Game {
  pub users: Vec<User>,
  pub bullets: Vec<Bullet>,
}

const WINDOW_WIDTH: i32 = 1080;
const WINDOW_HEIGHT: i32 = 720;
const SPEED: i32 = 10;
const FPS: u32 = 30;
impl Game {
  pub fn main(time: &mut Instant, out: &Sender, game: &Mutex<Game>) {
    if time.elapsed().as_millis() >= (1000. / (FPS as f32)) as u128 {
      let mut _game = game.lock().unwrap();
      _game.bullets = _game.bullets.iter().filter(|b| {
        b.x >= -10 && b.x <= WINDOW_WIDTH + 10 && b.y >= -10 && b.y <= WINDOW_HEIGHT + 10
      }).cloned().collect();
      for bullet in _game.bullets.iter_mut() {
        bullet.set_position();
      }
      out.broadcast(Message::from(_game.get_json())).unwrap();
      *time = Instant::now();
    }
  }

  pub fn add_user(&mut self, id: &String, x: i32, y: i32, degree: i32) {
    self.users.push(User {
      id: id.to_string(),
      x: x,
      y: y,
      degree: degree,
    });
  }

  pub fn remove_user(&mut self, id: &String) {
    self.users = self
      .users
      .iter()
      .filter(|&user| user.id != id.to_string())
      .cloned()
      .collect();
  }

  fn add_bullet(&mut self, x: i32, y: i32, mx: i32, my:i32) {
    self.bullets.push(Bullet {
      x: x,
      y: y,
      mx: mx,
      my: my,
    });
  }

  pub fn set_position(&mut self, json: Value, id: &String) {
    let mut users = self.users.clone();
    for user in users.iter_mut() {
      if user.id == format!("id-{}", id) {
        let f = |j: &Value, k| {
          let ret: u8 = match j[k].to_string().parse() {
            Ok(v) => v,
            Err(_) => 0,
          };
          ret
        };
        let left: u8 = f(&json, "left");
        let up: u8 = f(&json, "up");
        let right: u8 = f(&json, "right");
        let down: u8 = f(&json, "down");
        let add_degree = if left == 1 {
          -SPEED
        } else if right == 1 {
          SPEED
        } else {
          0
        };
        let rad: f32 =
          (f32::consts::PI * (user.degree + add_degree) as f32 / 180.) - f32::consts::PI / 2.;
        let (mut addx, mut addy) = (0, 0);
        if up == 1 {
          addx = (rad.cos() * SPEED as f32) as i32;
          addy = (rad.sin() * SPEED as f32) as i32;
        } else if down == 1 {
          addx = -(rad.cos() * SPEED as f32) as i32;
          addy = -(rad.sin() * SPEED as f32) as i32;
        }
        user.add(addx, addy, add_degree);

        if f(&json, "shot") == 1 {
          let bx = user.x + (rad.cos() * 30.) as i32;
          let by = user.y + (rad.sin() * 30.) as i32;
          self.add_bullet(bx, by, (rad.cos() * SPEED as f32) as i32, (rad.sin() * SPEED as f32) as i32);
        }
      }
    }
    self.users = users;
  }

  pub fn get_json(&self) -> String {
    let ret = serde_json::to_string(&self).unwrap();
    ret
  }
}

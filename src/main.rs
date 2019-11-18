extern crate ws;

use std::cell::{Cell, RefCell};
use std::f32;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use ws::{listen, CloseCode, Error, Handler, Handshake, Message, Result, Sender};

pub mod json_parse;
pub mod mgr;

struct Server {
  out: Arc<Mutex<Sender>>,
  count: Rc<Cell<u32>>,
  users: Arc<Mutex<mgr::Users>>,
}

impl Handler for Server {
  fn on_open(&mut self, _: Handshake) -> Result<()> {
    let out = self.out.lock().unwrap();
    let mut users = self.users.lock().unwrap();
    let id = format!("id-{}", out.connection_id().to_string());
    users.add_user(&id, 100, 100, 0);
    Ok(self.count.set(self.count.get() + 1))
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
    let out = self.out.lock().unwrap();
    println!("The number of live connection is {}", self.count.get());
    let json = json_parse::parse(msg.to_string()).unwrap();
    let mut users = self.users.lock().unwrap();
    for user in users.list.iter_mut() {
      if user.id == format!("id-{}", out.connection_id().to_string()) {
        let left: u8 = json["left"].to_string().parse().unwrap();
        let up: u8 = json["up"].to_string().parse().unwrap();
        let right: u8 = json["right"].to_string().parse().unwrap();
        let down: u8 = json["down"].to_string().parse().unwrap();
        let add_degree = if left == 1 {
          -5
        } else if right == 1 {
          5
        } else {
          0
        };
        let rad: f32 =
          (f32::consts::PI * (user.degree + add_degree) as f32 / 180.) - f32::consts::PI / 2.;
        let (mut addx, mut addy) = (0, 0);
        if up == 1 {
          addx = (rad.cos() * 5.) as i32;
          addy = (rad.sin() * 5.) as i32;
        } else if down == 1 {
          addx = -(rad.cos() * 5.) as i32;
          addy = -(rad.sin() * 5.) as i32;
        }
        user.add(addx, addy, add_degree);
      }
    }
    Ok(())
  }

  fn on_close(&mut self, code: CloseCode, reason: &str) {
    match code {
      CloseCode::Normal => println!("The client is done with the connection."),
      CloseCode::Away => println!("The client is leaving the site."),
      CloseCode::Abnormal => {
        println!("Closing handshake failed! Unable to obtain closing status from client.")
      }
      _ => println!("The client encountered an error: {}", reason),
    }
    let out = self.out.lock().unwrap();
    self
      .users
      .lock()
      .unwrap()
      .remove_user(&format!("id-{}", out.connection_id().to_string()));
    self.count.set(self.count.get() - 1)
  }

  fn on_error(&mut self, err: Error) {
    println!("The server encountered an error: {:?}", err);
  }
}

fn main() {
  let count = Rc::new(Cell::new(0));
  let users = Arc::new(Mutex::new(mgr::Users { list: vec![] }));
  let mut flag = false;
  listen("127.0.0.1:3012", |out| {
    let o = Arc::new(Mutex::new(out));
    if flag == false {
      let _o = o.clone();
      let _users = users.clone();
      thread::spawn(move || {
        println!("spawn");
        let mut time = Instant::now();
        loop {
          game_main(&mut time, &_o.lock().unwrap(), &_users.lock().unwrap());
        }
      });
      flag = true;
    }
    let ret = Server {
      out: o.clone(),
      count: count.clone(),
      users: users.clone(),
    };
    ret
  })
  .unwrap()
}

const FPS: u32 = 30;
fn game_main(time: &mut Instant, out: &Sender, users: &mgr::Users) {
  if time.elapsed().as_millis() >= (1000. / (FPS as f32)) as u128 {
    out.broadcast(Message::from(users.get_json())).unwrap();
    *time = Instant::now();
  }
}

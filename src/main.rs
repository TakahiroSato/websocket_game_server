extern crate ws;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use ws::{listen, CloseCode, Error, Handler, Handshake, Message, Result, Sender};

pub mod json_parse;
pub mod mgr;

struct Server {
  out: Sender,
  count: Rc<Cell<u32>>,
  users: RefCell<mgr::Users>,
}

impl Handler for Server {
  fn on_open(&mut self, _: Handshake) -> Result<()> {
    let id = format!("id-{}", self.out.connection_id().to_string());
    self.users.borrow_mut().add_user(&id);
    self.out.send(format!("open: Your ID is {}", &id)).unwrap();
    Ok(self.count.set(self.count.get() + 1))
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
    println!("The number of live connection is {}", self.count.get());
    let json = json_parse::parse(msg.to_string()).unwrap();
    for user in self.users.borrow_mut().list.iter_mut() {
      if user.id == format!("id-{}", self.out.connection_id().to_string()) {
        let x: i32 = json["x"].to_string().parse().unwrap();
        let y: i32 = json["y"].to_string().parse().unwrap();
        user.set_data(x, y);
        self.out.broadcast(Message::from(user.id.to_string())).unwrap();
        let _msg = format!("x={}, y={}", x.to_string(), y.to_string());
        self.out.broadcast(Message::from(_msg)).unwrap();
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
    self.count.set(self.count.get() - 1)
  }

  fn on_error(&mut self, err: Error) {
    println!("The server encountered an error: {:?}", err);
  }
}

fn main() {
  let count = Rc::new(Cell::new(0));
  listen("127.0.0.1:3012", |out| Server {
    out: out,
    count: count.clone(),
    users: RefCell::new(mgr::Users { list: vec![] })
  })
  .unwrap()
}

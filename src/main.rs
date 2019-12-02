extern crate ws;

use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use my_websocket::{game, json_parse};
use ws::{CloseCode, Error, Handler, Handshake, Message, Result, Sender, WebSocket};

struct Server {
    out: Arc<Mutex<Sender>>,
    count: Rc<Cell<u32>>,
    game: Arc<Mutex<game::Game>>,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let out = self.out.lock().unwrap();
        let mut g = self.game.lock().unwrap();
        let id = format!("id-{}", out.connection_id().to_string());
        g.add_user(&id);
        self.count.set(self.count.get() + 1);
        println!("count : {}", self.count.get());
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        let out = self.out.lock().unwrap();
        let json = json_parse::parse(msg.to_string()).unwrap();
        let mut g = self.game.lock().unwrap();
        g.set_position(json, &out.connection_id().to_string());
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
        self.game
            .lock()
            .unwrap()
            .remove_user(&format!("id-{}", out.connection_id().to_string()));
        self.count.set(self.count.get() - 1);
        println!("count : {}", self.count.get());
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

fn main() {
    let count = Rc::new(Cell::new(0));
    let g = Arc::new(Mutex::new(game::Game {
        users: vec![],
        bullets: vec![],
    }));
    let mut is_inited = false;
    let ws = WebSocket::new(move |out: Sender| {
        let out = Arc::new(Mutex::new(out.clone()));
        if !is_inited {
            let out = out.clone();
            let g = g.clone();
            thread::spawn(move || {
                let mut time = Instant::now();
                loop {
                    if time.elapsed().as_millis() >= (1000. / game::FPS) as u128 {
                        g.lock().unwrap().main(&out.lock().unwrap());
                        time = Instant::now();
                    }
                }
            });
            is_inited = true;
        }
        Server {
            out: out,
            count: count.clone(),
            game: g.clone(),
        }
    })
    .unwrap();
    ws.listen("0.0.0.0:3012").unwrap();
}

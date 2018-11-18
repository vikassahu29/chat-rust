extern crate iron;
extern crate server;
extern crate router;
extern crate persistent;
extern crate ws;

use iron::prelude::*;
use router::Router;
use server::*;
use persistent::Write;
use ws::*;
use std::thread;

struct Server {
    out: Sender
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("{ \"message\": \"Hello\"}")
    }
    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.out.broadcast(msg)
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}
fn main() {
    let application = application::init();
    let mut router = Router::new();
    handlers::attach(&mut router);
    thread::spawn(move || {
        listen("0.0.0.0:3001", |out| {
            Server {
                out: out
            }
        }).unwrap();
    });
    let mut chain = Chain::new(router);
    chain.link(Write::<handlers::Data>::both(application));
    Iron::new(chain)
        .http("0.0.0.0:3000").unwrap();
}
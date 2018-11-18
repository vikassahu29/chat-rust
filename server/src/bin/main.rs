extern crate iron;
extern crate server;
extern crate router;
extern crate persistent;

use iron::prelude::*;
use router::Router;
use server::*;
use persistent::Write;


fn main() {
    let application = application::init();
    let mut router = Router::new();
    handlers::attach(&mut router);
    let mut chain = Chain::new(router);
    chain.link(Write::<handlers::Data>::both(application));
    Iron::new(chain)
        .http("0.0.0.0:3000").unwrap();
}
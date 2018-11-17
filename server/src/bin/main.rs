extern crate iron;
extern crate server;
extern crate router;
extern crate persistent;
extern crate params;

use iron::prelude::*;
use iron::status;
use router::Router;
use server::*;
use server::models::*;
use persistent::Write;
use iron::typemap::Key;

#[derive(Copy, Clone)]
pub struct Data;
impl Key for Data { type Value = application::Application; }

fn register_handler(req: &mut Request) -> IronResult<Response> {
    let mutex = req.get::<Write<Data>>().unwrap();
    let mut app = mutex.lock().unwrap();
    use params::{Params, Value};

    let map = req.get_ref::<Params>().unwrap();
    
    let username: String;
    match map.get("username") {
        Some(&Value::String(ref u)) => username = u.to_string(),
        _ => {username = String::new()}
    };

    let password: String;
    
    match map.get("password") {
        Some(&Value::String(ref p)) => {
            password = p.to_string();
        }
        _ => { password = String::new() }
    };

    let user = User{
        username,
        password
    };

    let res = accounts::register_user(&mut app, &user);
    match res {
        Ok(_) => Ok(Response::with((status::Created,"Okay"))),
        Err(s) => Ok(Response::with((status::BadRequest, s)))
    }
}

fn login_handler(req: &mut Request) -> IronResult<Response> {
    let mutex = req.get::<Write<Data>>().unwrap();
    let mut app = mutex.lock().unwrap();
    use params::{Params, Value};

    let map = req.get_ref::<Params>().unwrap();
    
    let username: String;
    match map.get("username") {
        Some(&Value::String(ref u)) => username = u.to_string(),
        _ => {username = String::new()}
    };

    let password: String;
    
    match map.get("password") {
        Some(&Value::String(ref p)) => {
            password = p.to_string();
        }
        _ => { password = String::new() }
    };

    let user = User{
        username,
        password
    };

    let res = accounts::login_user(&mut app, &user);
    match res {
        Ok(_) => Ok(Response::with((status::Ok,"Okay"))),
        Err(s) => Ok(Response::with((status::BadRequest, s)))
    }
}


fn main() {
    let application = application::init();
    let mut router = Router::new();           // Alternative syntax:
    router.post("/register", register_handler, "register");
    router.post("/login", login_handler, "login");
    let mut chain = Chain::new(router);
    chain.link(Write::<Data>::both(application));

    // router.get("/", handler(&mut application), "index");        // let router = router!(index: get "/" => handler,
    // router.get("/:query", handler(&mut application), "query");  //                      query: get "/:query" => handler);
    Iron::new(chain)
        .http("0.0.0.0:3000").unwrap();
    
    
}
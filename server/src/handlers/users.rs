use ::models;
use iron::*;
use persistent::Write;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use std::io::Read;

pub fn register_handler(req: &mut Request) -> IronResult<Response> {
    let mutex = req.get::<Write<::handlers::Data>>().unwrap();
    let mut app = mutex.lock().unwrap();

    let mut s =  String::new();
    
    if let Err(_) = req.body.read_to_string(&mut s) {
        return Ok(Response::with(status::InternalServerError));
    }

    let user: models::User;

    match serde_json::from_str(&s) {
        Ok(u) => user = u,
        Err(_) => {
            return Ok(Response::with(status::InternalServerError));
        }
    }
    
    let res = ::accounts::register_user(&mut app, &user);
    match res {
        Ok(_) => {
            let res_data = json!({
                "message": "Account Created"
            });
            let mut resp = Response::with((status::Created, res_data.to_string()));
            resp.headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
            Ok(resp)
        },
        Err(s) =>{
            let res_data = json!({
                "message": s
            });
            let mut resp = Response::with((status::BadRequest, res_data.to_string()));
            resp.headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
            Ok(resp)            
        }
    }
}

pub fn login_handler(req: &mut Request) -> IronResult<Response> {
    let mutex = req.get::<Write<::handlers::Data>>().unwrap();
    
    let mut app = mutex.lock().unwrap();

    let mut s =  String::new();

    if let Err(_) = req.body.read_to_string(&mut s) {
        return Ok(Response::with(status::InternalServerError));
    }

    let user: ::models::User;

    match serde_json::from_str(&s) {
        Ok(u) => user = u,
        Err(_) => {
            return Ok(Response::with(status::InternalServerError));
        }
    }
    
    let res = ::accounts::login_user(&mut app, &user);

    match res {
        Ok(token) => {
            let res_data = json!({
                "token": token
            });
            let mut resp = Response::with((status::Ok, res_data.to_string()));
            resp.headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
            Ok(resp)
        },
        Err(s) =>{
            let res_data = json!({
                "message": s
            });
            let mut resp = Response::with((status::BadRequest, res_data.to_string()));
            resp.headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)])));
            Ok(resp)            
        }
    }
}
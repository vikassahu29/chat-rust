mod users;

use iron::typemap::Key;
use router::Router;
use application;

#[derive(Copy, Clone)]
pub struct Data;
impl Key for Data { type Value = application::Application; }

pub fn attach(router: &mut Router) {
    users::attach(router);
}
extern crate iron;
extern crate server;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use server::*;


fn main() {
    let _application = application::init();
    let mut router = Router::new();           // Alternative syntax:
    router.get("/", handler, "index");        // let router = router!(index: get "/" => handler,
    router.get("/:query", handler, "query");  //                      query: get "/:query" => handler);
    Iron::new(router)
        .http("0.0.0.0:3000").unwrap();
    
    
}

fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
}
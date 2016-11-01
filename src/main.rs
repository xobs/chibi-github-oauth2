extern crate inth_oauth2;
extern crate iron;

use inth_oauth2::Client;
use inth_oauth2::provider::GitHub;

use std::collections::HashMap;

use iron::prelude::*;
use iron::modifiers::Redirect;
use iron::{Url, status, Handler};

struct Router {
    // Routes here are simply matched with the url path.
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}

/*
fn main() {
    println!("{}", auth_uri);

    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();

    println!("{:?}", token);
}
*/

fn main() {
    let mut router = Router::new();
    router.add_route("new".to_string(), |_: &mut Request| {
        let client = Client::<GitHub>::new(
            String::from("01774654cd9a6051e478"),
            String::from("9f14d16d95d605e715ec1a9aecec220d2565fd5c"),
            Some(String::from("https://cmcenroe.me/oauth2-paste/"))
        );
        let url = Url::from_generic_url(client.auth_uri(Some("user"), None).unwrap()).unwrap();

        Ok(Response::with((status::Found, Redirect(url.clone()))))
    });

    router.add_route("get".to_string(), |_: &mut Request| {

        let client = Client::<GitHub>::new(
            String::from("01774654cd9a6051e478"),
            String::from("9f14d16d95d605e715ec1a9aecec220d2565fd5c"),
            Some(String::from("https://cmcenroe.me/oauth2-paste/"))
        );

        //let token = client.request_token(&Default::default(), code.trim()).unwrap();
        Ok(Response::with((status::Ok, "Got good")))
    });

    Iron::new(router).http("localhost:3000").unwrap();
}
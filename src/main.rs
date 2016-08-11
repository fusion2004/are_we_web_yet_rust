extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;
use serde_json::builder::ObjectBuilder;

mod models;

fn main() {
    let mut router = Router::new();
    router.get("/", handler);
    router.get("/users/:user_id", users_handler);

    let server_address = "localhost:3001";

    match Iron::new(router).http(server_address) {
        Ok(_) => println!("Listening on {}", server_address),
        Err(error) => println!("Error starting server: {}", error)
    }
}

fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Welcome to my rust service!")))
}

fn users_handler(req: &mut Request) -> IronResult<Response> {
    let ref user_id = req.extensions.get::<Router>().unwrap().find("user_id").unwrap_or("/");

    let mut attributes = ObjectBuilder::new();
    if user_id.to_string() == "123" {
        attributes = attributes.insert("name", "Mark");
    } else {
        attributes = attributes.insert("name", "John Doe");
    }

    let json = ObjectBuilder::new()
        .insert_object("data", |data| {
            data.insert("id", user_id)
                .insert("type", "users")
                .insert("attributes", attributes.build())
        });

    let response = serde_json::to_string(&json.build()).unwrap();

    Ok(Response::with((status::Ok, response)))
}

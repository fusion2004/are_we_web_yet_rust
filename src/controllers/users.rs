use iron::{Request, Response, IronResult};
use iron::status;
use router::Router;
use serde_json;
use serde_json::builder::ObjectBuilder;

pub fn show_handler(req: &mut Request) -> IronResult<Response> {
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

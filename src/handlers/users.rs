use gotham::state::State;
// use serde_json;
// use serde_json::builder::ObjectBuilder;
use diesel::prelude::*;
use crate::connection;
use crate::models::User;

pub fn index(state: State) -> (State, &'static str) {
    use crate::schema::users::dsl::*;

    let connection = connection::establish_connection();

    let results = users.limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);
        println!("----------\n");
        println!("{}", user.name);
        println!("{}", user.email);
    }

    (state, "")
}

// pub fn show(req: &mut Request) -> IronResult<Response> {
//     let ref user_id = req.extensions.get::<Router>().unwrap().find("user_id").unwrap_or("/");
//
//     let mut attributes = ObjectBuilder::new();
//     if user_id.to_string() == "123" {
//         attributes = attributes.insert("name", "Mark");
//     } else {
//         attributes = attributes.insert("name", "John Doe");
//     }
//
//     let json = ObjectBuilder::new()
//         .insert_object("data", |data| {
//             data.insert("id", user_id)
//                 .insert("type", "users")
//                 .insert("attributes", attributes.build())
//         });
//
//     let response = serde_json::to_string(&json.build()).unwrap();
//
//     Ok(Response::with((status::Ok, response)))
// }

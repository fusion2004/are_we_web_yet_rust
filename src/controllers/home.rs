use iron::{Request, Response, IronResult};
use iron::status;

pub fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Welcome to my rust service!")))
}

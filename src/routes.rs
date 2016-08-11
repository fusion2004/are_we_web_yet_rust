use router::Router;
use controllers;

pub fn router() -> Router {
    let mut router = Router::new();
    router.get("/", controllers::home::handler);
    router.get("/users/:user_id", controllers::users::show_handler);

    router
}

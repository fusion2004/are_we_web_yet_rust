use gotham::router::Router;
use gotham::router::builder::*;
use gotham::middleware::logger::RequestLogger;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use log::Level;
use crate::handlers::*;

// #[derive(Deserialize, StateData, StaticResponseExtender)]
struct UserPathParams {
    user_id: String
}

pub fn router() -> Router {
    let (chain, pipelines) = single_pipeline(
        new_pipeline()
            .add(RequestLogger::new(Level::Info))
            .build()
    );

    build_router(chain, pipelines, |route| {
        route.get_or_head("/").to(home::handler);

        route.scope("/users", |route| {
            route.get("/").to(users::index);
            // route.get("/:user_id")
            //     .with_path_extractor::<UserPathParams>()
            //     .to(user::show_handler);
        });
    })
}

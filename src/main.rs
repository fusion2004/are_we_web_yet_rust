extern crate clap;
extern crate iron;
extern crate router;
extern crate serde;
extern crate serde_json;

use clap::{Arg, App};
use iron::{Iron};

mod controllers;
mod models;
mod routes;

fn main() {
    let arg_matches = App::new("are_we_web_yet")
        .version("0.1")
        .author("Mark O. <fusion2004@gmail.com>")
        .about("Runs a simple web service")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Run on the specified port\n Default: 3000")
            .takes_value(true))
        .get_matches();

    let port = arg_matches.value_of("port").unwrap_or("3000").parse::<u16>().unwrap();
    let binding = "localhost";

    let router = routes::router();

    match Iron::new(router).http((binding, port)) {
        Ok(_) => println!("Listening on {}:{}", binding, port),
        Err(error) => println!("Error starting server: {}", error)
    }
}

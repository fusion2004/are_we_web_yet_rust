// #![feature(custom_derive, custom_attribute, plugin)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate clap;
extern crate gotham;
extern crate serde;
extern crate serde_json;
extern crate log;
extern crate env_logger;

use clap::{Arg, App};
use dotenv::dotenv;
use gotham::state::State;

mod connection;
mod schema;
mod models;
mod handlers;
mod routes;

fn main() {
    dotenv().ok();
    env_logger::init();

    let arg_matches = App::new("are_we_web_yet")
        .version("0.2.0")
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
    let addr = format!("127.0.0.1:{}", port);

    gotham::start(addr, routes::router());
}

// fn main() {
//     // Signal gets a value when the OS sent a INT or TERM signal.
//     let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
//     // When our work is complete, send a sentinel value on `sdone`.
//     let (sdone, rdone) = chan::sync(0);
//     // Run work.
//     ::std::thread::spawn(move || run(sdone));
//
//     // Wait for a signal or for work to be done.
//     chan_select! {
//         signal.recv() -> signal => {
//             match signal.unwrap() {
//                 Signal::INT | Signal::TERM => {
//                     println!("");
//                     println!("Shutting down")
//                 },
//                 _ => {}
//             }
//         },
//         rdone.recv() => {
//         }
//     }
// }
//
// fn run(_sdone: chan::Sender<()>) {
//     let arg_matches = App::new("are_we_web_yet")
//         .version("0.1")
//         .author("Mark O. <fusion2004@gmail.com>")
//         .about("Runs a simple web service")
//         .arg(Arg::with_name("port")
//             .short("p")
//             .long("port")
//             .value_name("PORT")
//             .help("Run on the specified port\n Default: 3000")
//             .takes_value(true))
//         .get_matches();
//
//     let port = arg_matches.value_of("port").unwrap_or("3000").parse::<u16>().unwrap();
//
//     let router = routes::router();
//
//     let (logger_before, logger_after) = Logger::new(None);
//
//     let mut chain = Chain::new(router);
//
//     // Link logger_before as your first before middleware.
//     chain.link_before(logger_before);
//
//     // Link logger_after as your *last* after middleware.
//     chain.link_after(logger_after);
//
//     match Iron::new(chain).http((binding, port)) {
//         Ok(_) => println!("Listening on {}:{}", binding, port),
//         Err(error) => println!("Error starting server: {}", error)
//     }
// }

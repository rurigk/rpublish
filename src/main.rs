use std::sync::Mutex;
use actix_web::{web, App, HttpServer, middleware};
extern crate termion;
use termion::{color};

mod helpers; // Initialization routines
mod rpublish; // RPublish system

mod public_endpoint; // Public pages related things
mod auth_endpoint; // Auth api
mod api_endpoint; // System api

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Check or create data directories
    match helpers::setup_system() {
        Ok(_) => {},
        Err(setup_error) => println!("{}Initialization failed: {}", color::Fg(color::Red), setup_error)
    }

    let data = web::Data::new(Mutex::new(
        rpublish::RPublishApp::default()
    ));

    println!("Starting the server");
    // Bind and start the server
    HttpServer::new(move || {
        App::new()
        .app_data( data.clone() )
        .wrap( middleware::NormalizePath::new(
            middleware::normalize::TrailingSlash::Trim
        ))
        .service(web::scope("/auth").configure(auth_endpoint::configure))
        .service(web::scope("/api").configure(api_endpoint::configure))
        .configure(public_endpoint::configure)
    }).bind("127.0.0.1:1337")?
    .run()
    .await
}
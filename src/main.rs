use std::sync::{Mutex};
use actix_web::{App, HttpServer, web};
extern crate termion;
use termion::{color};

mod helpers; // Initialization routines
mod rpublish; // RPublish system

mod handlers;
mod middleware;

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
        .wrap( actix_web::middleware::NormalizePath::new(
            actix_web::middleware::normalize::TrailingSlash::Trim
        ))
        .service(web::scope("/auth").configure(handlers::auth::configure))
        .service(web::scope("/api").configure(handlers::api::configure))
        .service(
            web::scope("/dashboard")
                .configure(handlers::dashboard::configure)
                .wrap(middleware::auth::LoggedIn)
        )
        .service(
            actix_files::Files::new("/public", "assets/public")
                .show_files_listing()
                .use_last_modified(true),
        )
        .configure(handlers::public::configure)
    }).bind("127.0.0.1:1337")?
    .run()
    .await
}
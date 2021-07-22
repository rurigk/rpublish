use std::sync::Mutex;

use actix_web::{web, App, HttpServer, Responder};
extern crate termion;
use termion::{color};

mod init;
mod rpublish;

async fn index(app: web::Data<Mutex<rpublish::RPublishApp>>) -> impl Responder {
    // Aquire app reference
    let mut app = app.lock().unwrap();

    let test_data = app.test();
    format!("Request number: {}", test_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Check or create data directories
    let setup_result = init::setup_system();
    match setup_result {
        Ok(_) => {},
        Err(setup_error) => {
            println!("{}Initialization failed: {}", color::Fg(color::Red), setup_error);
        }
    }

    let data = web::Data::new(Mutex::new(
        rpublish::RPublishApp{
            shared: String::new()
        }
    ));

    HttpServer::new(move || {
        App::new().app_data(data.clone()).service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ...so this handles requests for `GET /app/index.html`
                .route("/index", web::get().to(index)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
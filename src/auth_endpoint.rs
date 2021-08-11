use actix_web::{Responder, web};
use std::sync::Mutex;
use crate::rpublish;

pub fn configure (cfg: &mut web::ServiceConfig)
{
	cfg.route( "", web::get().to(home) )
	   .route( "/login", web::get().to(login) );
}

pub async fn home(app: web::Data<Mutex<rpublish::RPublishApp>>) -> impl Responder {
    // Aquire app reference
    let mut _app = app.lock().unwrap();

    format!("auth home")
}

pub async fn login(app: web::Data<Mutex<rpublish::RPublishApp>>) -> impl Responder {
    // Aquire app reference
    let mut _app = app.lock().unwrap();

    format!("auth login")
}
use actix_web::{Responder, web};
use std::sync::Mutex;
use crate::rpublish;

pub fn configure (cfg: &mut web::ServiceConfig)
{
	cfg.route("", web::get().to(home));
}

pub async fn home(app: web::Data<Mutex<rpublish::RPublishApp>>) -> impl Responder {
    // Aquire app reference
    let mut _app = app.lock().unwrap();

    "home".to_string()
}
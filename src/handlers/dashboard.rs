use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use std::{fs, sync::{Mutex, MutexGuard}};
use crate::rpublish::{self, RPublishApp};

pub fn configure (cfg: &mut web::ServiceConfig)
{
	cfg.route("", web::get().to(dashboard));
}

pub async fn dashboard(req: HttpRequest, app: web::Data<Mutex<rpublish::RPublishApp>>) -> impl Responder {
    // Aquire app reference
    let app = app.lock().unwrap();

    if !logged_in(&req, &app) {
        return HttpResponse::TemporaryRedirect()
            .header("Location", "/auth")
            .finish()
    }

    match fs::read_to_string("assets/templates/dashboard.html") {
        Ok(login_template) => HttpResponse::Ok().body(login_template),
        Err(_) => HttpResponse::InternalServerError().body("Failed to read template"),
    }
}

pub fn logged_in(req: &HttpRequest, app: &MutexGuard<RPublishApp>) -> bool{
    if let Some(sessid_cookie) = req.cookie("SESSID") {
        if app.identity_manager.sessions.validate(&sessid_cookie.value().to_string())
        {
            return true;
        }
        return false;
    } else {
        false
    }
}
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, cookie::Cookie, http, web};
use std::{fs, sync::Mutex};
use crate::rpublish;

use serde::{Deserialize};

use uuid::Uuid;

pub fn configure (cfg: &mut web::ServiceConfig)
{
	cfg.route( "", web::get().to(home) )
	   .route( "/logout", web::get().to(logout) )
       .route( "/login", web::get().to(login) )
       .route( "/login", web::post().to(login_post) );
}

pub async fn home(app: web::Data<Mutex<rpublish::RPublishApp>>) -> HttpResponse {
    // Aquire app reference
    let mut _app = app.lock().unwrap();

    HttpResponse::TemporaryRedirect()
        .header("Location", "/auth/login")
        .finish()
}

pub async fn login(req: HttpRequest, app: web::Data<Mutex<rpublish::RPublishApp>>) -> impl Responder {
    // Aquire app reference
    let app = app.lock().unwrap();

    if let Some(sessid_cookie) = req.cookie("SESSID") {
        if app.identity_manager.sessions.validate(&sessid_cookie.value().to_string())
        {
            return HttpResponse::TemporaryRedirect()
                .header("Location", "/dashboard")
                .finish()
        }
    }

    match fs::read_to_string("assets/templates/login.html") {
        Ok(login_template) => HttpResponse::Ok().body(login_template),
        Err(_) => HttpResponse::InternalServerError().body("Failed to read template"),
    }
}

pub async fn login_post(
    req: HttpRequest,
    app: web::Data<Mutex<rpublish::RPublishApp>>, 
    form: web::Form<LoginFormData>
) -> impl Responder {
    let mut app = app.lock().unwrap();
    let remote_ip = req.connection_info().remote_addr().unwrap_or_default().to_string();

    if let Some(sessid_cookie) = req.cookie("SESSID") {
        if app.identity_manager.sessions.validate(&sessid_cookie.value().to_string())
        {
            return HttpResponse::Forbidden().header("Location", "/dashboard").finish()
        }
    }

    let mut login_data = String::new();
    login_data.push_str(form.username.as_str());
    login_data.push_str(form.password.as_str());

    match app.identity_manager.users.get(form.username.as_str()) {
        Ok(user) => {
            match user.authenticate(form.password.as_str()) {
                Ok(()) => {
                    let uuid = Uuid::new_v4().to_simple();
                    let uuid2 = Uuid::new_v4().to_simple();

                    let sessid = format!("{}{}", uuid, uuid2);

                    app.identity_manager.sessions.create(
                        String::from(&sessid), 
                        String::from(&form.username), 
                        remote_ip
                    );

                    let cookie = Cookie::build("SESSID", sessid)
                        .path("/")
                        .secure(true)
                        .http_only(true)
                        .finish();

                    HttpResponse::Found()
                        .cookie(cookie)
                        .header(http::header::LOCATION, "/dashboard")
                        .finish().into_body()
                },
                Err(_) => HttpResponse::Unauthorized().body("Invalid credentials"),
            }
        },
        Err(_) => {
            HttpResponse::Unauthorized().body("Invalid credentials")
        },
    }
}

pub async fn logout(req: HttpRequest, app: web::Data<Mutex<rpublish::RPublishApp>>) -> impl Responder {
    // Aquire app reference
    let mut app = app.lock().unwrap();

    match req.cookie("SESSID") {
        Some(sessid_cookie) => {
            let sessid = sessid_cookie.value().to_string();
            if app.identity_manager.sessions.validate(&sessid)
            {
                app.identity_manager.sessions.invalidate(&sessid);

                return HttpResponse::TemporaryRedirect()
                    .header("Location", "/dashboard")
                    .finish()
            } else {
                return HttpResponse::TemporaryRedirect()
                .header("Location", "/auth/login")
                .finish()
            }
        },
        None => {
            return HttpResponse::TemporaryRedirect()
                .header("Location", "/auth/login")
                .finish()
        },
    }
}

#[derive(Deserialize)]
pub struct LoginFormData {
    username: String,
    password: String,
}
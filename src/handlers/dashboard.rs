use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, http, web};
use uuid::Uuid;
use std::{fs, sync::{Mutex, MutexGuard}};
use crate::rpublish::{self, RPublishApp};

pub fn configure (cfg: &mut web::ServiceConfig)
{
	cfg .route("", web::get().to(dashboard))
        .route("/articles", web::get().to(articles))
        .route("/settings", web::get().to(settings))
        .route("/article/new", web::get().to(new_article));
}

pub async fn dashboard() -> impl Responder {
    HttpResponse::Ok().body(get_dashboard("Dashboard", &String::from("dashboard")))
}

pub async fn articles() -> impl Responder {
    HttpResponse::Ok().body(get_dashboard("Articles", &String::from("articles")))
}

pub async fn settings() -> impl Responder {
    HttpResponse::Ok().body(get_dashboard("Settings",  &String::from("settings")))
}

pub async fn new_article(app: web::Data<Mutex<rpublish::RPublishApp>>) -> impl Responder {
    let mut app = app.lock().unwrap();
    let uuid = Uuid::new_v4().to_simple();
    app.articles_manager.create(uuid.to_string().as_str());
    // HttpResponse::Ok().body(get_dashboard("New Article",  &String::from("new_article")))
    HttpResponse::Found()
        .header(http::header::LOCATION, format!("{}{}", "/dashboard/article/edit/", uuid) )
        .finish().into_body()
}

fn get_dashboard(title: &str, section: &str) -> String
{
    match fs::read_to_string("assets/templates/dashboard.html") {
        Ok(login_template) => {
            let login_template = login_template.replace(
                "{{title}}", 
                title
            ).replace(
                "{{dashboard_items}}", 
                get_dashboard_items().as_str()
            ).replace(
                "{{section_content}}", 
                get_dashboard_section(section).as_str()
            );
            login_template
        },
        Err(_) => String::new(),
    }
}

fn get_dashboard_section(section: &str) -> String
{
    match fs::read_to_string(format!("assets/templates/dashboard/{}.html", section)) {
        Ok(login_template) => login_template,
        Err(_) => String::new(),
    }
}

fn get_dashboard_items() -> String
{
    match fs::read_to_string("assets/templates/dashboard_sidebar_items.html") {
        Ok(login_template) =>  login_template,
        Err(_) => String::new(),
    }
}
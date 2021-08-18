use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use std::{fs, sync::{Mutex, MutexGuard}};
use crate::rpublish::{self, RPublishApp};

pub fn configure (cfg: &mut web::ServiceConfig)
{
	cfg.route("", web::get().to(dashboard));
}

pub async fn dashboard() -> impl Responder {
    HttpResponse::Ok().body(get_dashboard(&String::from("dashboard")))
}

fn get_dashboard(section: &str) -> String
{
    match fs::read_to_string("assets/templates/dashboard.html") {
        Ok(login_template) => {
            let login_template = login_template.replace(
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
        Ok(login_template) => login_template,
        Err(_) => String::new(),
    }
}
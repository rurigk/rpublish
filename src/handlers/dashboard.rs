use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, http, web};
use uuid::Uuid;
use std::{fs, sync::{Mutex}};
use serde::{Serialize, Deserialize};
use crate::rpublish::{self};
use serde_json::json;

pub fn configure (cfg: &mut web::ServiceConfig)
{
	cfg .route("", web::get().to(dashboard))
        .route("/articles", web::get().to(articles))
        .route("/settings", web::get().to(settings))
        .route("/article/new", web::get().to(new_article))
        .route("/article/edit/{article_id}", web::get().to(edit_article_view))
        // Dashboard api
        .route("/api/article/{article_id}", web::get().to(api_get_article))
        .route("/api/article/{article_id}", web::put().to(api_update_article))
        .route("/api/article/{article_id}/publish", web::post().to(api_publish_article))
        
        .route("/api/articles/draft/{start_index}/{count}", web::get().to(api_list_draft_articles))
        .route("/api/articles/published/{start_index}/{count}", web::get().to(api_list_published_articles));
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

pub async fn new_article(
    req: HttpRequest,
    app: web::Data<Mutex<rpublish::RPublishApp>>
) -> impl Responder {
    let mut app = app.lock().unwrap();
    let sessid = req.cookie("SESSID").unwrap().value().to_string();

    let uuid = Uuid::new_v4().to_simple();
    let username = app.identity_manager.sessions.get_user(&sessid).unwrap();
    app.articles_manager.create(
        uuid.to_string().as_str(), 
        username.as_str()
    );
    // HttpResponse::Ok().body(get_dashboard("New Article",  &String::from("new_article")))
    HttpResponse::Found()
        .header(http::header::LOCATION, format!("{}{}", "/dashboard/article/edit/", uuid) )
        .finish().into_body()
}

pub async fn edit_article_view(
    app: web::Data<Mutex<rpublish::RPublishApp>>,
    info: web::Path<String>
) -> impl Responder {
    let mut app = app.lock().unwrap();
    let article_id: String = info.into_inner();
    match app.articles_manager.read_latest(&article_id) {
        Some(_) => {
            HttpResponse::Ok().body(get_dashboard(
                format!("Edit: {}", article_id).as_str(), 
                 &String::from("edit_article")
            ))
        },
        None => {
            HttpResponse::Found()
                .header(http::header::LOCATION,  "/dashboard/articles/" )
                .finish().into_body()
        },
    }
}

fn get_dashboard(title: &str, section: &str) -> String {
    match fs::read_to_string("assets/templates/dashboard.html") {
        Ok(dashboard_template) => {
            let dashboard_template = dashboard_template.replace(
                "{{title}}", 
                title
            ).replace(
                "{{dashboard_items}}", 
                get_dashboard_items().as_str()
            ).replace(
                "{{section_content}}", 
                get_dashboard_section(section).as_str()
            );
            dashboard_template
        },
        Err(_) => String::new(),
    }
}

fn get_dashboard_section(section: &str) -> String {
    match fs::read_to_string(format!("assets/templates/dashboard/{}.html", section)) {
        Ok(section_template) => section_template,
        Err(_) => String::new(),
    }
}

fn get_dashboard_items() -> String {
    match fs::read_to_string("assets/templates/dashboard_sidebar_items.html") {
        Ok(items_template) =>  items_template,
        Err(_) => String::new(),
    }
}

fn api_list_draft_articles (
    app: web::Data<Mutex<rpublish::RPublishApp>>,
    info: web::Path<(usize, usize)>
) -> HttpResponse {
    let mut  app = app.lock().unwrap();
    let limits = info.into_inner();
    let result = app.articles_manager.list_draft_articles(limits.0, limits.1);

    let json_response = json!({
        "articles": result.0,
        "total": result.1
    });

    HttpResponse::Ok().json(json_response)
}

fn api_list_published_articles (
    app: web::Data<Mutex<rpublish::RPublishApp>>,
    info: web::Path<(usize, usize)>
) -> HttpResponse {
    let mut  app = app.lock().unwrap();
    let limits = info.into_inner();
    let result = app.articles_manager.list_published_articles(limits.0, limits.1);

    let json_response = json!({
        "articles": result.0,
        "total": result.1
    });

    HttpResponse::Ok().json(json_response)
}

fn api_get_article (
    app: web::Data<Mutex<rpublish::RPublishApp>>, 
    info: web::Path<String>
) -> HttpResponse {
    let app = app.lock().unwrap();
    let article_id: String = info.into_inner();

    match app.articles_manager.read_latest(&article_id) {
        Some(article) => {
            HttpResponse::Ok().json(json!({
                "article": article.0,
                "status": article.1,
                "published": article.2,
                "published_date": article.3
            }))
        },
        None => {
            HttpResponse::NotFound().finish()
        },
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArticleUpdate {
    title: String,
    data: String
}

fn api_update_article (
    app: web::Data<Mutex<rpublish::RPublishApp>>, 
    info: web::Path<String>,
    article_update: web::Json<ArticleUpdate>
) -> HttpResponse {
    let mut app = app.lock().unwrap();
    let article_id: String = info.into_inner();

    println!("api_update_article {}", &article_id);

    match app.articles_manager.update(&article_id, &article_update.title, &article_update.data) {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(_) => {
            HttpResponse::NotFound().finish()
        },
    }
}

fn api_publish_article (
    app: web::Data<Mutex<rpublish::RPublishApp>>, 
    info: web::Path<String>
) -> HttpResponse {
    let mut app = app.lock().unwrap();
    let article_id: String = info.into_inner();

    match app.articles_manager.publish(&article_id) {
        Ok(_) => {
            HttpResponse::Ok().finish()
        },
        Err(_) => {
            HttpResponse::NotFound().finish()
        },
    }
}
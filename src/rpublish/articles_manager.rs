pub mod article;

use std::{fmt, fs};
use std::path::Path;

extern crate termion;
use termion::{color, style};

use crate::rpublish::metadata_cache::MetadataCache;
use crate::rpublish::articles_cache::ArticlesCache;

use self::article::Article;
use crate::helpers::write_json;

pub struct ArticlesManager{
    _metadata_cache: MetadataCache,
    _articles_cache: ArticlesCache,
    published_list: Vec<String>,
    draft_list: Vec<String>
}
impl ArticlesManager {
    pub fn new() -> ArticlesManager {
        Self::load_articles()
    }

    fn load_articles() -> ArticlesManager {
        let published_ids = Self::read_ids(Path::new("data/articles/published"));
        let draft_ids = Self::read_ids(Path::new("data/articles/draft"));

        Self {
            _metadata_cache: MetadataCache::new(),
            _articles_cache: ArticlesCache::new(),
            published_list: published_ids,
            draft_list: draft_ids
        }
    }

    fn read_ids(path: &Path) -> Vec<String>{
        match fs::read_dir(path) {
            Ok(published_files) => {
                let mut ids: Vec<String> = Vec::new();
                for file in published_files {
                    let file_path = file.unwrap().path();
                    if let Some(extension) = file_path.extension() {
                        if extension == "json" {
                            ids.push(String::from(file_path.file_stem().unwrap().to_str().unwrap()));
                        }
                    }
                }
                ids
            },
            Err(_) => {
                println!("{}Failed read files from{}", color::Fg(color::Red), path.to_str().unwrap());
                Vec::new()
            },
        }
    }

    pub fn create(&mut self, article_id: &str, author: &str) {
        let new_article = Article {
            title: String::from("Draft Article"),
            author: String::from(author),
            tags: Vec::new(),
            data: String::new(),
            created_date: chrono::offset::Utc::now(),
            update_date: chrono::offset::Utc::now(),
        };

        self.save_article(article_id, &new_article, ArticleStatus::Draft);
    }
    
    pub fn _read(&mut self, _article_id: &str) {

    }

    pub fn _read_metadata(&mut self, _article_id: &str) {

    }

    pub fn _update(&mut self, _article_id: &str) {

    }

    pub fn _delete(&mut self, _article_id: &str) {

    }

    fn save_article(&self, article_id: &str, article: &Article, status: ArticleStatus) {
        match serde_json::to_string(article) {
            Ok(json) => {
                match status {
                    ArticleStatus::Draft => {
                        match write_json(format!("data/articles/draft/{}.json", article_id).as_str(), json) {
                            Ok(_) => println!("{}Article draft saved", color::Fg(color::Cyan)),
                            Err(_) => println!("{}Failed to save draft article file", color::Fg(color::Red)),
                        }
                    },
                    ArticleStatus::Published => {
                        match write_json(format!("data/articles/published/{}.json", article_id).as_str(), json) {
                            Ok(_) => println!("{}Article saved to published", color::Fg(color::Cyan)),
                            Err(_) => println!("{}Failed to save article to publishs", color::Fg(color::Red)),
                        }
                    },
                    ArticleStatus::Archived => {
                        match write_json(format!("data/articles/archived/{}.json", article_id).as_str(), json) {
                            Ok(_) => println!("{}Article saved to archive", color::Fg(color::Cyan)),
                            Err(_) => println!("{}Failed to save article to archive", color::Fg(color::Red)),
                        }
                    },
                }
            },
            Err(_) => println!("{}Failed to serialize users file", color::Fg(color::Red))
        }
    }
}

pub enum ArticleStatus
{
    Draft,
    Published,
    Archived
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ArticleErrorKind
{
    ArticleAlreadyExist,
    ArticleNotFound
}

#[derive(Debug, Clone)]
pub struct ArticleError {
    kind: ArticleErrorKind
}

impl fmt::Display for ArticleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Article Error")
    }
}
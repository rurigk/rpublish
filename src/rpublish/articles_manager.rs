pub mod article;

use std::{fmt, fs};
use std::path::Path;

extern crate termion;
use actix_web::guard::Options;
use termion::{color, style};

use crate::rpublish::metadata_cache::MetadataCache;
use crate::rpublish::articles_cache::ArticlesCache;

use self::article::Article;
use crate::helpers::write_json;

pub struct ArticlesManager{
    // Metadata
    published_metadata_cache: MetadataCache,
    draft_metadata_cache: MetadataCache,

    articles_cache: ArticlesCache,
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
            published_metadata_cache: MetadataCache::new("published"),
            draft_metadata_cache: MetadataCache::new("draft"),
            articles_cache: ArticlesCache::new(),
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
    
    pub fn _list_published_articles (&mut self) {
        
    }

    fn read_article (file_path: &str) -> Option<Article> {
        match fs::read_to_string(file_path) {
            Ok(article_string) => {
                match serde_json::from_str::<Article>(article_string.as_str()) {
                    Ok(article) => {
                        Some(article)
                    },
                    Err(_) => {
                        None
                    }
                }
            },
            Err(_) => None,
        }
    }

    pub fn read_latest(&self, article_id: &str) -> Option<Article> {
        if self.draft_list.contains(&article_id.to_string()) {
            match Self::read_article(format!("data/articles/draft/{}.json", article_id).as_str()) {
                Some(article) => {
                    return Some(article)
                },
                None => None,
            }
        } else if self.published_list.contains(&article_id.to_string()) {
            match Self::read_article(format!("data/articles/published/{}.json", article_id).as_str()) {
                Some(article) => Some(article),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn _read_metadata(&mut self, _article_id: &str) {

    }

    pub fn update(&mut self, article_id: &str, title: &str, data: &str) -> Result<(), ArticleError> {
        match self.read_latest(&article_id) {
            Some(mut article) => {
                article.title = title.to_string();
                article.data = data.to_string();
                article.update_date = chrono::offset::Utc::now();
                self.save_article(article_id, &article, ArticleStatus::Draft);
                Ok(())
            },
            None => {
                Err(ArticleError{
                    kind: ArticleErrorKind::ArticleNotFound
                })
            },
        }
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

#[allow(dead_code)]
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
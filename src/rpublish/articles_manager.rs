pub mod article;

use std::{fmt, fs};
use std::path::Path;

extern crate termion;
use termion::{color};

use crate::rpublish::metadata_cache::MetadataCache;
use crate::rpublish::articles_cache::ArticlesCache;

use self::article::Article;
use crate::helpers::{write_json, move_file};

use super::metadata_cache::ArticleMetadata;

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
        let mut manager = Self::load_articles();
        manager.build_draft_metadata();
        manager.build_published_metadata();
        manager
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

    fn read_ids(path: &Path) -> Vec<String> {
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

    fn build_draft_metadata (&mut self) {
        for article_id in &self.draft_list {
            if !self.draft_metadata_cache.is_cached(article_id) {
                match self.read_from(article_id, ArticleStatus::Draft) {
                    Some(article) => {
                        self.draft_metadata_cache.set_metadata(article_id, &article);
                    },
                    None => {},
                }
            }
        }
    }

    fn build_published_metadata (&mut self) {
        for article_id in &self.published_list {
            if !self.published_metadata_cache.is_cached(article_id) {
                match self.read_from(article_id, ArticleStatus::Published) {
                    Some(article) => {
                        self.published_metadata_cache.set_metadata(article_id, &article);
                    },
                    None => {},
                }
            }
        }
    }

    fn rebuild_draft_metadata (&mut self, article_id: &str) {
        match self.read_from(article_id, ArticleStatus::Draft) {
            Some(article) => {
                self.draft_metadata_cache.set_metadata(article_id, &article);
            },
            None => {},
        }
    }
    
    fn rebuild_published_metadata (&mut self, article_id: &str) {
        match self.read_from(article_id, ArticleStatus::Published) {
            Some(article) => {
                self.published_metadata_cache.set_metadata(article_id, &article);
            },
            None => {},
        }
    }

    pub fn list_draft_articles (&mut self) -> Vec<&ArticleMetadata> {
        let mut articles_metadata: Vec<&ArticleMetadata> = Vec::new();
        for article_id in &self.draft_list {
            match &self.draft_metadata_cache.get_metadata(article_id) {
                Some(metadata) => {
                    articles_metadata.push(metadata);
                },
                None => {},
            }
        }
        articles_metadata
    }

    pub fn list_published_articles (&mut self) -> Vec<&ArticleMetadata> {
        let mut articles_metadata: Vec<&ArticleMetadata> = Vec::new();
        for article_id in &self.published_list {
            match &self.published_metadata_cache.get_metadata(article_id) {
                Some(metadata) => {
                    articles_metadata.push(metadata);
                },
                None => {},
            }
        }
        articles_metadata
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
                    }
                }
            },
            Err(_) => println!("{}Failed to serialize users file", color::Fg(color::Red))
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

        self.draft_metadata_cache.set_metadata(article_id, &new_article);
        self.save_article(article_id, &new_article, ArticleStatus::Draft);
    }

    pub fn read_latest (&self, article_id: &str) -> Option<Article> {
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

    pub fn read_from (&self, article_id: &str, status: ArticleStatus) -> Option<Article>{
        match status {
            ArticleStatus::Draft => {
                match Self::read_article(format!("data/articles/draft/{}.json", article_id).as_str()) {
                    Some(article) => {
                        return Some(article)
                    },
                    None => None,
                }
            },
            ArticleStatus::Published => {
                match Self::read_article(format!("data/articles/published/{}.json", article_id).as_str()) {
                    Some(article) => {
                        return Some(article)
                    },
                    None => None,
                }
            }
        }
    }

    pub fn update(&mut self, article_id: &str, title: &str, data: &str) -> Result<(), ArticleError> {
        match self.read_latest(&article_id) {
            Some(mut article) => {
                article.title = title.to_string();
                article.data = data.to_string();
                article.update_date = chrono::offset::Utc::now();
                self.draft_metadata_cache.set_metadata(article_id, &article);
                self.save_article(article_id, &article, ArticleStatus::Draft);

                let article_id_string = article_id.to_string();
                if !self.draft_list.contains(&article_id_string) {
                    self.draft_list.push(article_id_string);
                }
                Ok(())
            },
            None => {
                Err(ArticleError{
                    kind: ArticleErrorKind::ArticleNotFound
                })
            },
        }
    }

    pub fn _delete(&mut self, article_id: &str, status: ArticleStatus) {

    }

    pub fn publish(&mut self, article_id: &str) -> Result<(), std::io::Error>{
        match move_file(
            format!("data/articles/draft/{}.json", article_id).as_str(),
            format!("data/articles/published/{}.json", article_id).as_str()
        ) {
            Ok(_) => {
                let article_id_string = article_id.to_string();

                match self.draft_list.iter().position(|x| x == &article_id_string) {
                    Some(id_index) => {
                        self.draft_list.remove(id_index);
                    },
                    None => {},
                }

                if !self.published_list.contains(&article_id_string) {
                    self.published_list.push(article_id_string);
                }
                
                self.rebuild_published_metadata(article_id);
                Ok(())
            },
            Err(err) => Err(err),
        }
    }
}

#[allow(dead_code)]
pub enum ArticleStatus
{
    Draft,
    Published,
    //Archived
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
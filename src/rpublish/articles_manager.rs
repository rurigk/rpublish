pub mod article;

use std::collections::HashMap;
use std::io::ErrorKind;
use std::{fmt, fs};
use std::path::Path;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

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

    _articles_cache: ArticlesCache,
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
            _articles_cache: ArticlesCache::new(),
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
                    None => {
                        println!("Failed to read article when building draft metadata");
                    },
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
                    None => {
                        println!("Failed to read article when building published metadata");
                    },
                }
            }
        }
    }

    fn rebuild_metadata (&mut self, article_id: &str, origin: ArticleStatus) {
        match self.read_from(article_id, origin) {
            Some(article) => {
                let metadata_cache: &mut MetadataCache;

                if let ArticleStatus::Draft = origin {
                    metadata_cache = &mut self.draft_metadata_cache;
                } else {
                    metadata_cache = &mut self.published_metadata_cache;
                }
                
                metadata_cache.set_metadata(article_id, &article);
            },
            None => {
                println!("Failed to read article to rebuild metadata")
            },
        }
    }

    pub fn list_draft_articles (&mut self, start_index: usize, count: usize) -> (HashMap<String, &ArticleMetadata>, usize) {
        // Create a list for the returned refs
        let mut articles_metadata: HashMap<String, &ArticleMetadata> = HashMap::new();

        // Total of articles count
        let total = self.draft_list.len();
        
        // Normalize count
        let count = if count <= total {count} else {total};

        // Check if the start index is in bounds
        if start_index < total {
            let list_slice: &[String];

            // Get the correct slice
            if start_index < total - count {
                list_slice = &self.draft_list[start_index..start_index + count];
            } else {
                list_slice = &self.draft_list[start_index..];
            }

            for article_id in list_slice {
                match self.draft_metadata_cache.get_metadata(article_id) {
                    Some(metadata) => {
                        articles_metadata.insert(article_id.to_string(),metadata);
                    },
                    None => {
                        println!("Failed to get metadata of article when listing draft articles")
                    },
                }
            }
        }
        // Return metadata list
        (articles_metadata, total)
    }

    pub fn list_published_articles (&mut self, start_index: usize, count: usize) -> (HashMap<String, &ArticleMetadata>, usize) {
        // Create a list for the returned refs
        let mut articles_metadata: HashMap<String, &ArticleMetadata> = HashMap::new();

        // Total of articles count
        let total = self.published_list.len();

        let count = if count <= total {count} else {total};

        // Check if the start index is in bounds
        if start_index < total {
            let list_slice: &[String];

            // Get the correct slice
            if start_index < total - count {
                list_slice = &self.published_list[start_index..start_index + count];
            } else {
                list_slice = &self.published_list[start_index..];
            }

            for article_id in list_slice {
                match self.published_metadata_cache.get_metadata(article_id) {
                    Some(metadata) => {
                        articles_metadata.insert(article_id.to_string(),metadata);
                    },
                    None => {
                        println!("Failed to get metadata of article when listing published articles")
                    },
                }
            }
        }
        // Return metadata list
        (articles_metadata, total)
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
        self.draft_list.push(article_id.to_string());
    }

    pub fn read_latest (&self, article_id: &str) -> Option<(Article, ArticleStatus, bool, Option<DateTime<Utc>>)> {
        if self.draft_list.contains(&article_id.to_string()) {
            match Self::read_article(format!("data/articles/draft/{}.json", article_id).as_str()) {
                Some(article) => {
                    let is_published = self.published_list.contains(&article_id.to_string());
                    let published_date: Option<DateTime<Utc>>;

                    if is_published {
                        match self.published_metadata_cache.get_metadata(article_id) {
                            Some(metadata) => published_date = Some(metadata.update_date.to_owned()),
                            None => published_date = None,
                        }
                    } else {
                        published_date = None;
                    }

                    Some((
                        article, 
                        ArticleStatus::Draft, 
                        is_published,
                        published_date
                    ))
                },
                None => None,
            }
        } else if self.published_list.contains(&article_id.to_string()) {
            match Self::read_article(format!("data/articles/published/{}.json", article_id).as_str()) {
                Some(article) => {
                    let published_date = Some(article.update_date.to_owned());
                    Some((
                        article, 
                        ArticleStatus::Published, 
                        self.published_list.contains(&article_id.to_string()),
                        published_date
                    ))
                },
                None => None,
            }
        } else {
            None
        }
    }

    pub fn read_from (&self, article_id: &str, status: ArticleStatus) -> Option<Article>{
        match status {
            ArticleStatus::Draft => {
                Self::read_article(format!("data/articles/draft/{}.json", article_id).as_str())
            },
            ArticleStatus::Published => {
                Self::read_article(format!("data/articles/published/{}.json", article_id).as_str())
            }
        }
    }

    pub fn update(&mut self, article_id: &str, title: &str, data: &str) -> Result<(), ArticleError> {
        match self.read_latest(&article_id) {
            Some(mut article) => {
                article.0.title = title.to_string();
                article.0.data = data.to_string();
                article.0.update_date = chrono::offset::Utc::now();
                self.draft_metadata_cache.set_metadata(article_id, &article.0);
                self.save_article(article_id, &article.0, ArticleStatus::Draft);

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

    pub fn discard_changes(&mut self, article_id: &str) -> Result<(), std::io::Error> {
        let is_published = self.published_list.contains(&article_id.to_string());
        if is_published {
            match self.delete_article(article_id, ArticleStatus::Draft) {
                Ok(_) => {
                    Ok(())
                },
                Err(error) => Err(error),
            }
        } else {
            Err(std::io::Error::new(ErrorKind::NotFound, "Article is not published"))
        }
    }

    pub fn delete(&mut self, article_id: &str) -> Result<(), std::io::Error> {
        match self.delete_article(article_id, ArticleStatus::Draft) {
            Ok(_) => {
                match self.delete_article(article_id, ArticleStatus::Published) {
                    Ok(_) => {
                        Ok(())
                    },
                    Err(error) => Err(error),
                }
            },
            Err(error) => Err(error),
        }
    }

    pub fn publish(&mut self, article_id: &str) -> Result<(), std::io::Error>{
        self.move_article(article_id, ArticleStatus::Draft, ArticleStatus::Published)
    }

    pub fn unpublish(&mut self, article_id: &str) -> Result<(), std::io::Error>{

        if self.draft_list.contains(&article_id.to_string()) {
            // Just remove the published version and keep the draft
            self.delete_article(article_id, ArticleStatus::Published)
        } else if self.published_list.contains(&article_id.to_string()) {
            // We need to move the published article to the draft section
            self.move_article(article_id, ArticleStatus::Published, ArticleStatus::Draft)
        } else {
            Err(std::io::Error::new(ErrorKind::NotFound, "Article not found"))
        }
    }

    fn delete_article (&mut self, article_id: &str, origin: ArticleStatus) -> Result<(), std::io::Error> {
        let origin_str: &str;
        let list: &mut Vec<String>;

        if let ArticleStatus::Draft = origin {
            origin_str = "draft";
            list = &mut self.draft_list;
        } else {
            origin_str = "published";
            list = &mut self.published_list;
        }
        
        // Remove the id from the published list
        let article_id_string = article_id.to_string();
        match list.iter().position(|x| x == &article_id_string) {
            Some(id_index) => {
                list.remove(id_index);
            },
            None => {
                println!("Failed to find article_id index to remove it from the origin list after article deletion")
            },
        }

        // Remove published file
        match fs::remove_file(format!("data/articles/{}/{}.json", origin_str, article_id)) {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn move_article (&mut self, article_id: &str, origin: ArticleStatus, target: ArticleStatus) -> Result<(), std::io::Error> {
        let origin_str: &str;
        let target_str: &str;
        let origin_list: &mut Vec<String>;
        let target_list: &mut Vec<String>;
        
        if let ArticleStatus::Draft = origin {
            origin_str = "draft";
            target_str = "published";
            origin_list = &mut self.draft_list;
            target_list = &mut self.published_list;
        } else {
            origin_str = "published";
            target_str = "draft";
            origin_list = &mut self.published_list;
            target_list = &mut self.draft_list;
        }

        match move_file(
            format!("data/articles/{}/{}.json", origin_str, article_id).as_str(),
            format!("data/articles/{}/{}.json", target_str, article_id).as_str()
        ) {
            Ok(_) => {
                let article_id_string = article_id.to_string();

                match origin_list.iter().position(|x| x == &article_id_string) {
                    Some(id_index) => {
                        origin_list.remove(id_index);
                    },
                    None => {
                        println!("Failed to find article_id index to remove it from the origin list")
                    },
                }

                if !target_list.contains(&article_id_string) {
                    target_list.push(article_id_string);
                }

                match self.read_from (article_id, target) {
                    Some(mut article) => {
                        article.update_date = chrono::offset::Utc::now();
                        self.save_article(article_id, &article, target);
                    },
                    None => {
                        println!("Failed to read article to update the date after move")
                    },
                }
                
                self.rebuild_metadata(article_id,target);
                Ok(())
            },
            Err(err) => Err(err),
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ArticleStatus
{
    Draft,
    Published,
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
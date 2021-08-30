use std::{collections::HashMap, fs};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

use crate::{helpers::write_json};

use super::articles_manager::article::Article;

pub struct MetadataCache{
    pub path: PathBuf,
    pub articles: HashMap<String, ArticleMetadata>
}

impl MetadataCache {
    pub fn new(relative_path: &str) -> MetadataCache {
        let mut path = PathBuf::new();
        path.push("data/cache/metadata");
        path.push(relative_path);
        Self::load_from_disk(path)
    }

    // Load existing cache files
    fn load_from_disk(path: PathBuf) -> MetadataCache {
        match fs::read_dir(&path) {
            Ok(metadata_files) => {
                let mut cache = Self {
                    path: path,
                    articles: HashMap::new()
                };
                for file in metadata_files {
                    let file_path = file.unwrap().path();
                    if let Some(extension) = file_path.extension() {
                        if extension == "json" {
                            let article_id = String::from(file_path.file_stem().unwrap().to_str().unwrap());
                            match fs::read_to_string(&file_path) {
                                Ok(metadata_string) => {
                                    match serde_json::from_str::<ArticleMetadata>(metadata_string.as_str()) {
                                        Ok(article_metadata) => {
                                            cache.articles.insert(article_id, article_metadata);
                                        },
                                        Err(_) => {
                                            println!("Failed to parse {}.json", article_id);
                                        }
                                    }
                                },
                                Err(_) => println!("Cannot read {} as string", file_path.to_str().unwrap()),
                            }
                        }
                    }
                }
                cache
            },
            Err(_) => {
                Self {
                    path: path,
                    articles: HashMap::new()
                }
            },
        }
    }

    // Save cache to disk
    pub fn save_to_disk (&self) {
        for (article_id, article_metadata) in &self.articles {
            match serde_json::to_string(article_metadata) {
                Ok(json) => {
                    let mut metadata_path = self.path.to_owned();
                    metadata_path.push(format!("{}.json", &article_id));
                    match write_json(metadata_path.to_str().unwrap(), json) {
                        Ok(_) => println!("Metadata cache of {} saved", &article_id),
                        Err(_) => println!("Failed to save metadata of {}", &article_id),
                    }
                },
                Err(_) => println!("Failed to serialize metadata of {}", &article_id)
            }
        }
    }

    // Metadata cache methods

    // Check if article has metadata cached
    pub fn is_cached(&self, article_id: &str) -> bool {
        self.articles.contains_key(article_id)
    }
    
    // Get metadata of article
    pub fn get_metadata(&self, article_id: &str) -> Option<&ArticleMetadata> {
        self.articles.get(article_id)
    }

    // Add or update article metadata
    pub fn set_metadata(&mut self, article_id: &str, article: &Article) {
        if self.articles.contains_key(article_id) {
            match self.articles.get_mut(article_id) {
                Some(metadata) => {
                    metadata.title = article.title.to_owned();
                    metadata.author = article.author.to_owned();
                    metadata.tags = article.tags.to_owned();
                    metadata.update_date = article.update_date.to_owned();
                    self.save_to_disk();
                }
                None => {
                    println!("Cannot get a mutable ref to article metadata");
                },
            }
        } else {
            self.articles.insert(article_id.to_string(), ArticleMetadata {
                title: article.title.to_owned(),
                author: article.author.to_owned(),
                tags: article.tags.to_owned(),
                created_date: article.created_date.to_owned(),
                update_date: article.update_date.to_owned()
            });
            self.save_to_disk();
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArticleMetadata {
    pub title: String,
    pub author: String,
    pub tags: Vec<String>,
    pub created_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>
}
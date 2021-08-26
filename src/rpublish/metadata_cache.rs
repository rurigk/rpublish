use std::{collections::HashMap, fs};
use actix_web::guard::Options;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

use crate::helpers::write_json;

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

    pub fn is_cached(&self, article_id: &str) -> bool {
        self.articles.contains_key(article_id)
    }

    pub fn get_cache_of(&self, article_id: &str) -> Option<&ArticleMetadata> {
        self.articles.get(article_id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArticleMetadata {
    title: String,
    author: String,
    tags: Vec<String>
}
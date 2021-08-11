use crate::rpublish::metadata_cache::MetadataCache;
use crate::rpublish::articles_cache::ArticlesCache;

pub struct ArticlesManager{
    metadata_cache: MetadataCache,
    articles_cache: ArticlesCache
}
impl ArticlesManager {
    pub fn new() -> ArticlesManager {
        ArticlesManager{
            metadata_cache: MetadataCache::new(),
            articles_cache: ArticlesCache::new()
        }
    }

    pub fn _create(&mut self, _article_id: &str) {

    }
    
    pub fn _read(&mut self, _article_id: &str) {

    }

    pub fn _read_metadata(&mut self, _article_id: &str) {

    }

    pub fn _update(&mut self, _article_id: &str) {

    }

    pub fn _delete(&mut self, _article_id: &str) {

    }
}
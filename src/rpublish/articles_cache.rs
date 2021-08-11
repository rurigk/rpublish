pub struct ArticlesCache{

}
impl ArticlesCache {
    pub fn new() -> ArticlesCache {
        ArticlesCache{}
    }

    pub fn is_cached(&mut self, article_id: &str) -> bool {
        false
    }
}
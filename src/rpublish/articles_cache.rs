pub struct ArticlesCache{

}
impl ArticlesCache {
    pub fn new() -> ArticlesCache {
        ArticlesCache{}
    }

    pub fn _is_cached(&mut self, _article_id: &str) -> bool {
        false
    }
}
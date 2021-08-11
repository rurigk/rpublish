pub struct MetadataCache{

}
impl MetadataCache {
    pub fn new() -> MetadataCache {
        MetadataCache{}
    }

    pub fn is_cached(&mut self, article_id: &str) -> bool {
        false
    }
}
pub struct MetadataCache{

}
impl MetadataCache {
    pub fn new() -> MetadataCache {
        MetadataCache{}
    }

    pub fn _is_cached(&mut self, _article_id: &str) -> bool {
        false
    }
}
pub struct Article{
    title: String,
    author: String,
    data: String,
    tags: Vec<String>,
    created_date: DateTime<Utc>,
    update_date: DateTime<Utc>
}
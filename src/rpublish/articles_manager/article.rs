use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Article{
    pub title: String,
    pub author: String,
    pub data: String,
    pub tags: Vec<String>,
    pub created_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>
}
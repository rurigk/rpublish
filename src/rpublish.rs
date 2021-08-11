pub mod article;
pub mod article_metadata;
pub mod articles_manager;
pub mod articles_cache;
pub mod metadata_cache;
pub mod identity_manager;

use articles_manager::{ArticlesManager};
use identity_manager::IdentityManager;

pub struct RPublishApp
{
    pub identity_manager: IdentityManager,
    pub articles_manager: ArticlesManager
}

impl RPublishApp {
    pub fn default() -> RPublishApp {
        RPublishApp {
            identity_manager: IdentityManager::new(),
            articles_manager: ArticlesManager::new()
        }
    }
}


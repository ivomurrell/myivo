use crate::scrapers::backloggd::{self, Backloggd};

use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct RootTemplate {
    game: Option<Backloggd>,
}

impl RootTemplate {
    pub async fn new() -> RootTemplate {
        RootTemplate {
            game: backloggd::cached_fetch().await,
        }
    }
}

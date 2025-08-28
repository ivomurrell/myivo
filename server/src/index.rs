use crate::scrapers::backloggd::Backloggd;

use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct RootTemplate {
    game: Option<Backloggd>,
}

impl RootTemplate {
    pub async fn new() -> RootTemplate {
        RootTemplate {
            game: Backloggd::fetch()
                .await
                .map_err(|error| tracing::warn!(%error, "failed to scrape Backloggd"))
                .ok(),
        }
    }
}

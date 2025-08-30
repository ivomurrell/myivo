use crate::scrapers::backloggd::{self, Backloggd};
use crate::scrapers::letterboxd::{self, Letterboxd};

use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct RootTemplate {
    game: Option<Backloggd>,
    movie: Option<Letterboxd>,
}

impl RootTemplate {
    pub async fn new() -> RootTemplate {
        let (game, movie) = tokio::join!(backloggd::cached_fetch(), letterboxd::cached_fetch(),);
        RootTemplate { game, movie }
    }
}

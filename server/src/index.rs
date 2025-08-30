use crate::scrapers::{
    apple_music::{self, AppleMusic},
    backloggd::{self, Backloggd},
    letterboxd::{self, Letterboxd},
};

use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "index.html")]
pub struct RootTemplate {
    game: Option<Backloggd>,
    movie: Option<Letterboxd>,
    song: Option<AppleMusic>,
}

impl RootTemplate {
    pub async fn new() -> RootTemplate {
        let (game, movie, song) = tokio::join!(
            backloggd::cached_fetch(),
            letterboxd::cached_fetch(),
            apple_music::cached_fetch()
        );
        RootTemplate { game, movie, song }
    }
}

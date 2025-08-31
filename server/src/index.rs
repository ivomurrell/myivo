use crate::scrapers::{
    apple_music::{self, AppleMusic, AppleMusicClient},
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
    pub async fn new(apple_music_client: &AppleMusicClient) -> RootTemplate {
        let (game, movie, song) = tokio::join!(
            backloggd::cached_fetch(),
            letterboxd::cached_fetch(),
            apple_music::cached_fetch(apple_music_client)
        );
        RootTemplate { game, movie, song }
    }
}

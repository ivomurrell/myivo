use askama::Template;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Artist {
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Image {
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Attributes {
    #[serde(rename = "nowplaying")]
    pub now_playing: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Track {
    pub artist: Artist,
    pub image: Vec<Image>,
    pub name: String,
    #[serde(rename = "@attr")]
    pub attributes: Option<Attributes>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecentTracks {
    pub track: Vec<Track>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scrobble {
    #[serde(rename = "recenttracks")]
    pub recent_tracks: RecentTracks,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Scrobble(Scrobble),
    Error(Error),
}

#[derive(Template, Debug, Clone, PartialEq)]
#[template(path = "scrobble.html")]
pub struct ScrobblesTemplate {
    pub intro: &'static str,
    pub now_playing: String,
    pub image: Option<String>,
    pub srcset: Option<String>,
}

impl ScrobblesTemplate {
    pub fn new(scrobble: &Scrobble) -> ScrobblesTemplate {
        let latest_track = &scrobble.recent_tracks.track[0];
        let srcset = latest_track.image.get(0..3).map(|images| {
            format!(
                "{}, {} 2x, {} 3x",
                images[0].text, images[1].text, images[2].text
            )
        });
        let text_intro = if latest_track
            .attributes
            .as_ref()
            .is_some_and(|attr| attr.now_playing == "true")
        {
            "Now playing: "
        } else {
            "Last played: "
        };
        let now_playing = format!("{} - {}", latest_track.name, latest_track.artist.text);

        ScrobblesTemplate {
            intro: text_intro,
            now_playing,
            image: latest_track.image.first().map(|image| image.text.clone()),
            srcset,
        }
    }
}

use askama::Template;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ScrobbleArtist {
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScrobbleImage {
    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScrobbleAttributes {
    #[serde(rename = "nowplaying")]
    pub now_playing: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScrobbleTrack {
    pub artist: ScrobbleArtist,
    pub image: Vec<ScrobbleImage>,
    pub name: String,
    #[serde(rename = "@attr")]
    pub attributes: Option<ScrobbleAttributes>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScrobbleRecentTracks {
    pub track: (ScrobbleTrack,),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scrobble {
    #[serde(rename = "recenttracks")]
    pub recent_tracks: ScrobbleRecentTracks,
}

#[derive(Template, Debug, Clone, PartialEq)]
#[template(path = "scrobble.html")]
pub struct ScrobblesTemplate {
    pub intro: &'static str,
    pub now_playing: String,
    pub image: Option<String>,
    pub srcset: Option<String>,
}

pub fn scrobble_partial(scrobble: Scrobble) -> ScrobblesTemplate {
    let (latest_track,) = scrobble.recent_tracks.track;
    let srcset = latest_track.image.get(0..3).map(|images| {
        format!(
            "{}, {} 2x, {} 3x",
            images[0].text, images[1].text, images[2].text
        )
    });
    let text_intro = if latest_track
        .attributes
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
        image: latest_track
            .image
            .into_iter()
            .next()
            .map(|image| image.text),
        srcset,
    }
}

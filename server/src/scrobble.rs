use maud::{html, Markup};
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
    pub now_playing: bool,
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
    pub track: Vec<ScrobbleTrack>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Scrobble {
    #[serde(rename = "recenttracks")]
    pub recent_tracks: ScrobbleRecentTracks,
}

pub fn scrobble_partial(scrobble: &Scrobble) -> Markup {
    let latest_track = &scrobble.recent_tracks.track[0];
    let srcset = format!(
        "{}, {} 2x, {} 3x",
        latest_track.image[0].text, latest_track.image[1].text, latest_track.image[2].text
    );
    let text_intro = if latest_track
        .attributes
        .as_ref()
        .map_or(false, |attr| attr.now_playing)
    {
        "Now playing: "
    } else {
        "Last played: "
    };
    let now_playing = format!("{} - {}", latest_track.name, latest_track.artist.text);

    html! {
        .bar-container {
            img .bar-cover
                src=(latest_track.image[0].text)
                alt="Cover art"
                srcset=(srcset);

            p .bar-text-intro {
                (text_intro)
            }

            p .bar-text-music {
                (now_playing)
            }
        }
    }
}

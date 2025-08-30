use std::{sync::LazyLock, time::Duration};

use anyhow::Context;
use cached::proc_macro::once;
use reqwest::{Client, Url};
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Letterboxd {
    pub name: String,
    pub poster: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ImageUrlMetadata {
    url: String,
}

struct Extracted {
    name: String,
    image_url: Url,
}

impl Letterboxd {
    pub async fn fetch() -> anyhow::Result<Self> {
        let client = Client::new();
        let html = client
            .get("https://letterboxd.com/ivom/films/diary/")
            .send()
            .await
            .context("failed to fetch Letterboxd page")?
            .text()
            .await
            .context("failed to get HTML text")?;
        let Extracted { name, image_url } = Self::parse_html(&html)?;

        let image_url_data: ImageUrlMetadata = client
            .get(image_url.clone())
            .send()
            .await
            .with_context(|| format!("failed to fetch image metadata from URL {}", image_url))?
            .json()
            .await
            .context("failed to parse image metadata")?;

        Ok(Self {
            name,
            poster: image_url_data.url,
        })
    }

    fn parse_html(html: &str) -> anyhow::Result<Extracted> {
        static FIRST_ENTRY_SEL: LazyLock<Selector> =
            LazyLock::new(|| Selector::parse(".diary-entry-row:first-child").unwrap());
        static NAME_SEL: LazyLock<Selector> = LazyLock::new(|| Selector::parse(".name").unwrap());
        static POSTER_COMPONENT_SEL: LazyLock<Selector> =
            LazyLock::new(|| Selector::parse(".react-component:has(> .poster)").unwrap());

        let document = Html::parse_document(html);

        let first_entry = document
            .select(&FIRST_ENTRY_SEL)
            .next()
            .context("couldn't find any journal entries")?;
        let name = first_entry
            .select(&NAME_SEL)
            .next()
            .context("couldn't find name element")?
            .text()
            .next()
            .context("name element didn't have any text")?
            .to_owned();
        let poster_component = first_entry
            .select(&POSTER_COMPONENT_SEL)
            .next()
            .context("couldn't find post component")?;

        let image_url = Self::build_image_url(poster_component)?;

        Ok(Extracted { name, image_url })
    }

    fn build_image_url(poster_component: ElementRef) -> anyhow::Result<Url> {
        let film_path = poster_component
            .attr("data-item-link")
            .context("poster component didn't have an image URL path")?;
        let cache_key = poster_component.attr("data-cache-busting-key");
        let image_size = 230;
        let image_url = format!(
            "https://letterboxd.com{}/poster/std/{}/",
            film_path, image_size
        );
        let mut image_url =
            Url::parse(&image_url).with_context(|| format!("failed to parse URL {}", image_url))?;
        if let Some(cache_key) = cache_key {
            image_url.query_pairs_mut().append_pair("k", cache_key);
        }

        Ok(image_url)
    }
}

#[once(time = 1800, option = false)]
pub async fn cached_fetch() -> Option<Letterboxd> {
    Letterboxd::fetch()
        .await
        .map_err(|error| tracing::warn!(?error, "failed to scrape Letterboxd"))
        .ok()
}

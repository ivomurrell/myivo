use std::sync::LazyLock;

use anyhow::Context;
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct Backloggd {
    pub name: String,
    pub image: String,
}

impl Backloggd {
    pub async fn fetch() -> anyhow::Result<Self> {
        static FIRST_ENTRY_SEL: LazyLock<Selector> =
            LazyLock::new(|| Selector::parse(".journal_entry:first-child").unwrap());
        static NAME_SEL: LazyLock<Selector> =
            LazyLock::new(|| Selector::parse(".game-name a").unwrap());
        static IMAGE_SEL: LazyLock<Selector> =
            LazyLock::new(|| Selector::parse(".card-img").unwrap());

        let html = reqwest::get("https://backloggd.com/u/cherryfunk/journal")
            .await
            .context("failed to fetch Backloggd page")?
            .text()
            .await
            .context("failed to get HTML text")?;
        let document = Html::parse_document(&html);

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
        let image = first_entry
            .select(&IMAGE_SEL)
            .next()
            .context("couldn't find image element")?
            .attr("src")
            .context("image element didn't have src attribute")?
            .to_owned();

        Ok(Self { name, image })
    }
}

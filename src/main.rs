use anyhow::{Context, Result};
use fantoccini::{Client, Locator};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CastMember {
    pub image_url: String,
    pub name: String,
    pub episodes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MovieData {
    pub cast: Vec<CastMember>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let c = Client::new("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    let url = std::env::args()
        .next()
        .context("Required argument missing: cast page url")
        .unwrap();

    c.goto(&url).await?;

    let mut writer = csv::Writer::from_path("data.csv")?;

    let credits = c
        .find(Locator::Css(".people.credits"))
        .await?
        .find_all(Locator::Css("li"))
        .await?;

    for credit in credits {
        let image_url = credit.find(Locator::Css("a")).await?;
        let image_url = image_url.attr("href").await?.unwrap();
        let image_url = format!("https://www.themoviedb.org{}", image_url);
        let name = credit
            .find(Locator::Css("div > .info > span > p > a"))
            .await?;
        let name = name.text().await?;

        let episodes = credit
            .find(Locator::Css("div > .info > span > .character > span"))
            .await?;
        let episodes = episodes.text().await?;
        let episodes = episodes
            .replace("(", "")
            .replace(")", "")
            .replace(" Episodes", "")
            .replace(" Episode", "");
        //dbg!(episodes);
        let episodes = episodes.parse::<i32>().unwrap();

        writer.serialize(CastMember {
            image_url,
            name,
            episodes,
        })?;
    }

    c.close().await?;

    Ok(())
}

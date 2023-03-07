#![warn(clippy::all, clippy::pedantic)]

pub mod news_api {
    use std::error::Error;
    use colour::{dark_green, dark_yellow, grey};
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Articles {
        articles: Vec<Article>
    }

#[derive(Debug, Deserialize)]
pub struct Article {
    title: String,
    url: String,
    description: Option<String>
}

pub fn get_headlines() -> Result<Articles, Box<dyn Error>> {
    let url: &str = "https://newsapi.org/v2/top-headlines?category=science&apiKey=API_KEY";
    let response = ureq::get(url).call()?.into_string()?;
    let articles: Articles = serde_json::from_str(&response)?;
    Ok(articles)
}

    pub fn render_headlines(articles: &Articles) {
        for article in &articles.articles {
            dark_green!("> {}\n", article.title);
            dark_yellow!("> {}\n", article.url);
            /* Description can be Null */
            match &article.description {
                Some(description) => grey!("> {}\n\n", description),
                None => (),
            }
        }
    }
}

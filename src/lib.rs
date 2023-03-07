pub mod news_api {
    use std::error::Error;
    use colour::{dark_green, dark_yellow};
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Articles {
        articles: Vec<Article>
    }

#[derive(Debug, Deserialize)]
pub struct Article {
    title: String,
    url: String
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
            dark_yellow!("> {}\n\n", article.url);
        }
    }
}

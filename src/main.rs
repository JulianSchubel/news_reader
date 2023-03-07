use news_reader::news_api::*;

fn main() {
    let headlines = get_headlines();
    match headlines {
        Ok(x) => render_headlines(&x),
        Err(e) => println!("{}", e),
    }
}

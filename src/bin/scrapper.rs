use scraper::{Html, Selector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let document =
        Html::parse_document(&"https://www.lbma.org.uk/prices-and-data/lbma-precious-metal-prices");
    // println!("rendered_html: {}", document.conte);

    let title_selector = Selector::parse("h1.main-title").unwrap();

    // 6. Iterate and extract the data
    for element in document.select(&title_selector) {
        let title_text = element.text().collect::<Vec<_>>().join(" ");
        println!("Found Title: {}", title_text.trim());
    }

    Ok(())
}

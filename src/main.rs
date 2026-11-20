use headless_chrome::{Browser, LaunchOptions};
use scraper::{Html, Selector};
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Configure and launch the headless browser
    let ops = LaunchOptions::default_builder()
        .headless(true) // Set to false if you want to visually debug
        .build()?;

    let browser = Browser::new(ops)?;
    let tab = browser.new_tab()?;

    // 2. Navigate to the JavaScript-dependent website
    println!("Navigating to target page...");
    tab.navigate_to("https://www.lbma.org.uk/prices-and-data/lbma-precious-metal-prices")?;

    // 3. Wait for the JavaScript execution to complete
    // You can wait for a specific element to load in the DOM
    // tab.wait_for_element("div.dynamic-content-loaded")?;
    // tab.wait_for_element("js-priceswidget-current.mb-6.text-18.md:text-22.font-light.min-h-8.font-medium.font-extrabold")?;

    // Alternatively, sleep for a rigid duration if needed:
    // std::thread::sleep(Duration::from_secs(3));

    // 4. Extract the fully rendered HTML content
    let rendered_html = tab.get_content()?;

    // 5. Parse the HTML using the 'scraper' crate
    let document = Html::parse_document(&rendered_html);
    // println!("rendered_html: {}", document.conte);

    let title_selector = Selector::parse("h1.main-title").unwrap();

    // 6. Iterate and extract the data
    for element in document.select(&title_selector) {
        let title_text = element.text().collect::<Vec<_>>().join(" ");
        println!("Found Title: {}", title_text.trim());
    }

    Ok(())
}

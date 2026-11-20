// use chromiumoxide::browser::{Browser, BrowserConfig};
// use chromiumoxide::handler::viewport::Viewport;
// use futures::StreamExt;
// use std::time::Duration;
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // 1. Configure the browser to run headlessly
//     let config = BrowserConfig::builder()
//         .request_timeout(Duration::from_secs(30))
//         .window_size(1920, 1080)
//         .build()?;
//
//     // 2. Launch the browser and capture the event loop handle
//     let (mut browser, mut handler) = Browser::launch(config).await?;
//
//     // Spawn the browser event handler thread in the background
//     let handle = tokio::task::spawn(async move {
//         while let Some(h) = handler.next().await {
//             if let Err(err) = h {
//                 eprintln!("Browser handler loop error: {}", err);
//                 break;
//             }
//         }
//     });
//
//     // 3. Open a new browser page/tab
//     let page = browser.new_page("https://example.com").await?;
//
//     // 4. Wait for the dynamic element to appear in the DOM
//     let selector = "div.dynamic-content-listing > h3";
//     // let element = page.wait_for_selector(selector).await?;
//     let element = page.wait_for_element(selector).await?;
//
//     // 5. Extract the inner text or HTML content
//     let text = element.inner_text().await?.unwrap_or_default();
//     println!("Extracted dynamic text: {}", text);
//
//     // 6. Gracefully close the tab and the browser browser instance
//     page.close().await?;
//     browser.close().await?;
//     handle.await?;
//
//     Ok(())
// }

fn main() {

}
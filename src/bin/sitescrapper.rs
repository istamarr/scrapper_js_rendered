use sitescraper;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let html = sitescraper::http::get("http://example.com/").await.unwrap();
    let dom = sitescraper::parse_html(&html).unwrap();
    // let filtered_dom = dom.filter("div");
    // println!("{}", filtered_dom.get_inner_html());
    println!("{:?}", dom.get_inner_html());

    Ok(())
}

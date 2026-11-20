use headless_chrome::{Browser, LaunchOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::new(LaunchOptions::default_builder().headless(true).build()?)?;

    let tab = browser.new_tab()?;
    // tab.navigate_to("https://example.com")?;
    // <div class="js-priceswidget-current mb-6 text-18 md:text-22 font-light min-h-8"
    // aria-live="polite"><span class="font-medium">AM </span>
    //     <strong class="font-extrabold">$4,120.35</strong><span class="ml-3 text-14 opacity-70">08/10</span></div>
    // div.js-priceswidget-current.mb-6.text-18.md:text-22.font-light.min-h-8.aria-live.strong.font-extrabold
    // ./target/release/obscura fetch https://www.lbma.org.uk/prices-and-data/lbma-precious-metal-prices --eval "document.body.prices.antialiased.h-full.debug-screens.div.bg-turquoise.text-white.relative.z-20.div.container.div.row.div.flex.items-center.justify-between.py-2.div.flex.justify-end.flex-1.a.items-center.hidden.gap-2.font-medium.text-white.fnav:flex.text-15.span.px-2.py-1.uppercase.border.border-turquoise-light.rounded-full"
    // ./target/release/obscura fetch https://www.lbma.org.uk/prices-and-data/lbma-precious-metal-prices --eval "document.body.div.div.div.div.div.a.span"
    tab.navigate_to("https://www.lbma.org.uk/prices-and-data/lbma-precious-metal-prices")?;
    // Uob
    // ./target/release/obscura fetch https://www.lbma.org.uk/prices-and-data/lbma-precious-metal-prices --eval "document.body.div.div.div.div.div.div.div.div.div.div.div.table.tbody.tr.td.p"
    // https://www.uobgroup.com/online-rates/gold-and-silver-prices.page
    //
    //     Ylg
    // ./target/release/obscura fetch https://www.lbma.org.uk/prices-and-data/lbma-precious-metal-prices --eval "document.body.main.section.div.div.div.div.div.div.div.div"
    // https://www.ylgfutures.co.th/en/gold-price
    //
    //     Hatadinata
    // body.main.div.section.div.div.div.div.div.table.tbody.tr.td.div.div.div.span.span
    // ./target/release/obscura fetch https://www.lbma.org.uk/prices-and-data/lbma-precious-metal-prices --eval "document.body.main.div.section.div.div.div.div.div.table.tbody.tr.td.div.div.div.span.span"
    // harga dasar 0.1 gram
    // https://gadaihartadinataabadi.com/harga-emas/
    //
    //     Bbte
    //
    // Kurs bi
    // ./target/release/obscura fetch https://www.lbma.org.uk/prices-and-data/lbma-precious-metal-prices --eval "document.body.form.div.div.div.div.div.div.div.div.div.table.tbody.tr.td.text-right"
    // https://www.bi.go.id/id/statistik/informasi-kurs/transaksi-bi/default.aspx
    //
    //     Others open apis kurs
    //     https://1forge.com/api
    //     https://publicapis.dev/category/currency-exchange
    //     https://developers.bri.co.id/id/docs/valuta-asing-2
    //     https://exchangeratesapi.io/
    //     https://apidevportal.bi.go.id/snap/
    //     https://www.exchangerate-api.com/
    //     https://openexchangerates.org/
    //     https://github.com/bachors/Kurs-API
    //     https://www.bi.go.id/id/statistik/informasi-kurs/transaksi-bi/default.aspx
    //
    //     Check gold price apus kurs, yg ada xauus
    //     https://www.exchangerate-api.com/docs/overview
    //
    // Sumber scrapping
    // https://www.goldrate24.com/gold-prices/asia/indonesia/gram/
    //     Munculin di widget plugin
    // https://goldprice.org/gold-price-indonesia.html

    // Wait until an element matching the CSS selector is drawn
    // let element = tab.wait_for_element("div.dynamic-content-listing")?;

    let inner_text = element.get_inner_text()?;
    println!("Scraped: {}", inner_text);

    Ok(())
}

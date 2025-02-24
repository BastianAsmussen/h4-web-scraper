use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let url = "https://www.proshop.dk/";
    let response = client.get(url).send()?.text()?;
    let document = Html::parse_document(&response);

    // TODO: Refine these selectors.
    let title_selector = Selector::parse("h1[data-type=\"product\"]").unwrap();
    let specs_selector = Selector::parse("p.pb-2").unwrap();
    let price_selector =
        Selector::parse("div.site-currency-attention.site-currency-campaign").unwrap();

    for element in document.select(&title_selector) {
        let title = element
            .value()
            .attr("product-display-name")
            .unwrap_or("N/A")
            .to_string();

        let specs = document
            .select(&specs_selector)
            .next()
            .map(|e| e.inner_html().trim().to_string())
            .unwrap_or("N/A".to_string());

        let price = document
            .select(&price_selector)
            .next()
            .map(|e| e.inner_html().trim().to_string())
            .unwrap_or("N/A".to_string());

        println!(
            "Product: {}\nSpecifications: {}\nPrice: {}\n",
            title, specs, price
        );
    }

    Ok(())
}

use crate::{Context, Error};

/// Test web scraping (for development purposes only)
#[poise::command(slash_command)]
pub async fn scrape(_ctx: Context<'_>) -> Result<(), Error> {
    let response = reqwest::blocking::get("https://www.scrapingcourse.com/ecommerce/");
    let html_content = response.unwrap().text().unwrap();

    let _document = scraper::Html::parse_document(&html_content);

    println!("{}", html_content);
    // let html_product_selector = scraper::Selector::parse("li.product").unwrap();
    // let html_products = document.select(&html_product_selector);

    Ok(())
}

use crate::data::models::ListingData;
use crate::error::AppError;
use scraper::{Html, Selector, ElementRef};
use reqwest;


//TODO move these selectors to a better place (assigned constants?) or store them in a struct idk
const CARD_DIV: &str = "div[data-cy='l-card']";
const TITLE: &str = "h6.css-1wxaaza";
const PRICE: &str = "p.css-13afqrm";
const LOCATION_DATE: &str = "p.css-1mwdrlh";
const CONDITION: &str = "span.css-3lkihg";
const FEATURED: &str = "div[data-testid='adCard-featured']";
const DELIVERY_BADGE: &str = "div[data-testid='card-delivery-badge']";
const SAFETY_BADGE: &str = "img[alt='Safety badge']";

pub async fn fetch_and_parse_listings(query: &str) -> Result<Vec<ListingData>, AppError> {
    let url = format!("https://www.olx.pl/oferty/q-{}/", query.replace(" ", "-"));
    let html_content = fetch_html(&url).await?;
    let fragment = Html::parse_document(&html_content);
    let card_selector = Selector::parse(CARD_DIV).unwrap();
    fragment.select(&card_selector)
        .map(|element| extract_listing_data(&element))
        .collect()
}           

async fn fetch_html(url: &str) -> Result<String, AppError> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

fn extract_listing_data(element: &ElementRef) -> Result<ListingData, AppError> {
    let get_text = |selector: &str| -> String {
        element.select(&Selector::parse(selector).unwrap()).next()
            .map_or_else(String::new, |el| el.text().collect::<String>().trim().to_string())
    };

    let get_attr = |selector: &str, attr: &str| -> String {
        element.select(&Selector::parse(selector).unwrap()).next()
            .and_then(|el| el.value().attr(attr))
            .unwrap_or_default().to_string()
    };
    

    Ok(ListingData {

        //TODO sanitize price 
        id: element.value().attr("id").unwrap_or_default().to_string(),
        url: get_attr("a", "href"),
        title: get_text(TITLE),
        price: get_text(PRICE),
        image_url: get_attr("img", "src"),
        location_date: get_text(LOCATION_DATE),
        condition: get_text(CONDITION),
        is_featured: element.select(&Selector::parse(FEATURED).unwrap()).next().is_some(),
        has_delivery: element.select(&Selector::parse(DELIVERY_BADGE).unwrap()).next().is_some(),
        has_safety_badge: element.select(&Selector::parse(SAFETY_BADGE).unwrap()).next().is_some(),
    })
}

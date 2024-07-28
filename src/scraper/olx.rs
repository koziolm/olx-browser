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

pub async fn fetch_and_parse_listings(query: &str, page: u32) -> Result<(Vec<ListingData>, u32), AppError> {
    let url = if page == 1 {
        format!("https://www.olx.pl/oferty/q-{}/", query.replace(" ", "-"))
    } else {
        format!("https://www.olx.pl/oferty/q-{}/?page={}", query.replace(" ", "-"), page)
    };

    let html_content = fetch_html(&url).await?;
    let fragment = Html::parse_document(&html_content);
    
    let total_pages = extract_last_page_number(&fragment)?;
    
    let card_selector = Selector::parse(CARD_DIV).unwrap();
    let listings: Result<Vec<ListingData>, AppError> = fragment.select(&card_selector)
        .map(|element| extract_listing_data(&element))
        .collect();

    Ok((listings?, total_pages))
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

    // Extract price, ignoring nested elements
    let extract_price = |selector: &str| -> String {
        element.select(&Selector::parse(selector).unwrap()).next()
            .map_or_else(String::new, |el| {
                el.children()
                    .filter_map(|child| {
                        if let scraper::Node::Text(text) = child.value() {
                            Some(text.trim().to_string())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
                    .trim()
                    .to_string()
            })
    };

    Ok(ListingData {
        id: element.value().attr("id").unwrap_or_default().to_string(),
        url: get_attr("a", "href"),
        title: get_text(TITLE),
        price: extract_price(PRICE),
        image_url: get_attr("img", "src"),
        location_date: get_text(LOCATION_DATE),
        condition: get_text(CONDITION),
        is_featured: element.select(&Selector::parse(FEATURED).unwrap()).next().is_some(),
        has_delivery: element.select(&Selector::parse(DELIVERY_BADGE).unwrap()).next().is_some(),
        has_safety_badge: element.select(&Selector::parse(SAFETY_BADGE).unwrap()).next().is_some(),
    })
}


fn extract_last_page_number(fragment: &Html) -> Result<u32, AppError> {
    let selector = Selector::parse("li.pagination-item").unwrap();
    
    fragment.select(&selector)
        .filter_map(|element| {
            element.select(&Selector::parse("a").unwrap()).next()
                .and_then(|a| a.text().next())
                .and_then(|text| text.parse::<u32>().ok())
        })
        .max()
        .ok_or(AppError::ParseError("Could not find last page number".to_string()))
}
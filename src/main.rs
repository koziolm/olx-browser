use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use scraper::{Html, Selector, ElementRef};
use reqwest;
use tokio;

const CARD_DIV: &str = "div[data-cy='l-card']";
const TITLE: &str = "h6.css-1wxaaza";
const PRICE: &str = "p.css-13afqrm";
const LOCATION_DATE: &str = "p.css-1mwdrlh";
const CONDITION: &str = "span.css-3lkihg";
const FEATURED: &str = "div[data-testid='adCard-featured']";
const DELIVERY_BADGE: &str = "div[data-testid='card-delivery-badge']";
const SAFETY_BADGE: &str = "img[alt='Safety badge']";

#[derive(Debug)]

struct ListingData {
    id: String,
    url: String,
    title: String,
    price: String,
    image_url: String,
    location_date: String,
    condition: String,
    is_featured: bool,
    has_delivery: bool,
    has_safety_badge: bool,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Fetch HTML content
    let url = "https://www.olx.pl/oferty/q-karta-graficzna/";  // Example URL, replace with the actual URL you want to scrape
    let html_content = fetch_html(url).await?;

    // Parse HTML and extract listings
    let fragment = Html::parse_document(&html_content);
    let card_selector = Selector::parse(CARD_DIV).unwrap();
    let listings: Vec<ListingData> = fragment.select(&card_selector)
        .map(extract_listing_data)
        .collect();

    // App state
    let mut selected_listing = 0;

    // Main loop
    loop {
        terminal.draw(|f| ui(f, &listings, selected_listing))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => selected_listing = (selected_listing + 1) % listings.len(),
                KeyCode::Up => {
                    selected_listing = if selected_listing > 0 {
                        selected_listing - 1
                    } else {
                        listings.len() - 1
                    }
                }
                _ => {}
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

fn ui<B: Backend>(f: &mut Frame<B>, listings: &[ListingData], selected_listing: usize) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = listings
        .iter()
        .enumerate()
        .map(|(i, listing)| {
            let content = vec![Spans::from(listing.title.clone())];
            if i == selected_listing {
                ListItem::new(content).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                ListItem::new(content)
            }
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Listings"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_widget(items, chunks[0]);

    let selected = &listings[selected_listing];
    let info = vec![
        Spans::from(vec![
            Span::raw("ID: "),
            Span::styled(&selected.id, Style::default().fg(Color::Yellow)),
        ]),
        Spans::from(vec![
            Span::raw("Price: "),
            Span::styled(&selected.price, Style::default().fg(Color::Green)),
        ]),
        Spans::from(vec![
            Span::raw("Condition: "),
            Span::styled(&selected.condition, Style::default().fg(Color::Cyan)),
        ]),
        Spans::from(vec![
            Span::raw("Location and Date: "),
            Span::styled(&selected.location_date, Style::default().fg(Color::Magenta)),
        ]),
        Spans::from(vec![
            Span::raw("Featured: "),
            Span::styled(selected.is_featured.to_string(), Style::default().fg(Color::Red)),
        ]),
        Spans::from(vec![
            Span::raw("Has Delivery: "),
            Span::styled(selected.has_delivery.to_string(), Style::default().fg(Color::Blue)),
        ]),
        Spans::from(vec![
            Span::raw("Has Safety Badge: "),
            Span::styled(selected.has_safety_badge.to_string(), Style::default().fg(Color::LightRed)),
        ]),
    ];

    let details = Paragraph::new(info)
        .block(Block::default().borders(Borders::ALL).title("Details"));

    f.render_widget(details, chunks[1]);
}


fn extract_listing_data(element: ElementRef) -> ListingData {
    let title_selector = Selector::parse(TITLE).unwrap();
    let price_selector = Selector::parse(PRICE).unwrap();
    let location_date_selector = Selector::parse(LOCATION_DATE).unwrap();
    let condition_selector = Selector::parse(CONDITION).unwrap();
    let featured_selector = Selector::parse(FEATURED).unwrap();
    let delivery_badge_selector = Selector::parse(DELIVERY_BADGE).unwrap();
    let safety_badge_selector = Selector::parse(SAFETY_BADGE).unwrap();

    ListingData {
        id: element.value().attr("id").unwrap_or_default().to_string(),
        url: element.select(&Selector::parse("a").unwrap()).next()
            .and_then(|el| el.value().attr("href"))
            .unwrap_or_default().to_string(),
        title: element.select(&title_selector).next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default(),
        price: element.select(&price_selector).next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default(),
        image_url: element.select(&Selector::parse("img").unwrap()).next()
            .and_then(|el| el.value().attr("src"))
            .unwrap_or_default().to_string(),
        location_date: element.select(&location_date_selector).next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default(),
        condition: element.select(&condition_selector).next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .unwrap_or_default(),
        is_featured: element.select(&featured_selector).next().is_some(),
        has_delivery: element.select(&delivery_badge_selector).next().is_some(),
        has_safety_badge: element.select(&safety_badge_selector).next().is_some(),
    }
}           
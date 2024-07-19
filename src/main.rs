use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
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

struct AppState {
    listings: Vec<ListingData>,
    selected_listing: usize,
    error: Option<String>,
    query: String,
    input: String,
    input_mode: InputMode,
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState {
        listings: Vec::new(),
        selected_listing: 0,
        error: None,
        query: String::from("karta graficzna"),
        input: String::new(),
        input_mode: InputMode::Normal,
    };

    fetch_listings(&mut app_state).await?;

    loop {
        terminal.draw(|f| ui(f, &app_state))?;

        if let Event::Key(key) = event::read()? {
            match app_state.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('e') => {
                        app_state.input_mode = InputMode::Editing;
                        app_state.input = app_state.query.clone();
                    }
                    KeyCode::Down => {
                        if !app_state.listings.is_empty() {
                            app_state.selected_listing = (app_state.selected_listing + 1) % app_state.listings.len();
                        }
                    },
                    KeyCode::Up => {
                        if !app_state.listings.is_empty() {
                            app_state.selected_listing = if app_state.selected_listing > 0 {
                                app_state.selected_listing - 1
                            } else {
                                app_state.listings.len() - 1
                            };
                        }
                    },
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app_state.query = app_state.input.clone();
                        app_state.input_mode = InputMode::Normal;
                        fetch_listings(&mut app_state).await?;
                    }
                    KeyCode::Char(c) => {
                        app_state.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app_state.input.pop();
                    }
                    KeyCode::Esc => {
                        app_state.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

async fn fetch_listings(app_state: &mut AppState) -> Result<(), Box<dyn Error>> {
    let url = format!("https://www.olx.pl/oferty/q-{}/", app_state.query.replace(" ", "-"));
    match fetch_and_parse_listings(&url).await {
        Ok(listings) => {
            app_state.listings = listings;
            app_state.error = None;
        }
        Err(e) => app_state.error = Some(format!("Error fetching listings: {}", e)),
    }
    Ok(())
}

async fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

async fn fetch_and_parse_listings(url: &str) -> Result<Vec<ListingData>, Box<dyn Error>> {
    let html_content = fetch_html(url).await?;
    let fragment = Html::parse_document(&html_content);
    let card_selector = Selector::parse(CARD_DIV).unwrap();
    fragment.select(&card_selector)
        .map(extract_listing_data)
        .collect()
}

fn extract_listing_data(element: ElementRef) -> Result<ListingData, Box<dyn Error>> {
    let title_selector = Selector::parse(TITLE).unwrap();
    let price_selector = Selector::parse(PRICE).unwrap();
    let location_date_selector = Selector::parse(LOCATION_DATE).unwrap();
    let condition_selector = Selector::parse(CONDITION).unwrap();
    let featured_selector = Selector::parse(FEATURED).unwrap();
    let delivery_badge_selector = Selector::parse(DELIVERY_BADGE).unwrap();
    let safety_badge_selector = Selector::parse(SAFETY_BADGE).unwrap();

    Ok(ListingData {
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
    })
}

fn ui<B: Backend>(f: &mut Frame<B>, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let query_display = Paragraph::new(format!("Active Query: {}", app_state.query))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Query"));
    f.render_widget(query_display, chunks[0]);

    if let Some(error) = &app_state.error {
        let error_msg = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(error_msg, chunks[1]);
    }

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[2]);

    let items: Vec<ListItem> = app_state.listings
        .iter()
        .enumerate()
        .map(|(i, listing)| {
            let content = vec![Spans::from(listing.title.clone())];
            if i == app_state.selected_listing {
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

    f.render_widget(items, main_chunks[0]);

    if !app_state.listings.is_empty() {
        let selected = &app_state.listings[app_state.selected_listing];
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

        f.render_widget(details, main_chunks[1]);
    }

    if app_state.input_mode == InputMode::Editing {
        let input = Paragraph::new(app_state.input.as_ref())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Enter Search Query"));
        let area = centered_rect(60, 20, f.size());
        f.render_widget(Clear, area);
        f.render_widget(input, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
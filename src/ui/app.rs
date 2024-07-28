use crate::data::models::ListingData;
use crate::error::AppError;
use crate::scraper::olx::fetch_and_parse_listings;
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncWriteExt;           

pub struct App {
    pub listings: Vec<ListingData>,
    pub selected_index: usize,
    pub show_dialog: bool,
    pub query: String,
    pub current_page: u32,
    pub total_pages: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            listings: Vec::new(),
            selected_index: 0,
            show_dialog: true,
            query: String::from(" "),
            current_page: 1,
            total_pages: 1,
        }
    }

    pub fn toggle_dialog(&mut self) {
        self.show_dialog = !self.show_dialog;
    }

    pub fn input_char(&mut self, c: char) {
        self.query.push(c);
    }

    pub fn backspace(&mut self) {
        self.query.pop();
    }


    pub async fn next_page(&mut self) -> Result<(), AppError> {
        if self.current_page < self.total_pages {
            self.current_page += 1;
            self.fetch_current_page().await?;
        }
        Ok(())
    }

    pub async fn prev_page(&mut self) -> Result<(), AppError> {
        if self.current_page > 1 {
            self.current_page -= 1;
            self.fetch_current_page().await?;
        }
        Ok(())
    }

    async fn fetch_current_page(&mut self) -> Result<(), AppError> {
        let (listings, total_pages) = fetch_and_parse_listings(&self.query, self.current_page).await?;
        self.listings = listings;
        self.total_pages = total_pages;
        self.selected_index = 0;
        Ok(())
    }


    async fn dump_all_pages_csv(&self) -> Result<(), AppError> {
        let filename = "listings.csv";
        let mut file = AsyncFile::create(filename).await?;
        let mut wtr = csv::WriterBuilder::new().from_writer(vec![]);
    
        // Write header
        wtr.write_record(&[
            "ID", "URL", "Title", "Price", "Image URL", "Location/Date",
            "Condition", "Is Featured", "Has Delivery", "Has Safety Badge"
        ])?;
    
        // Fetch and write data for all pages
        for page in 1..=self.total_pages {
            let (listings, _) = fetch_and_parse_listings(&self.query, page).await?;
            for listing in listings {
                wtr.write_record(&[
                    &listing.id,
                    &listing.url,
                    &listing.title,
                    &listing.price,
                    &listing.image_url,
                    &listing.location_date,
                    &listing.condition,
                    &listing.is_featured.to_string(),
                    &listing.has_delivery.to_string(),
                    &listing.has_safety_badge.to_string(),
                ])?;
            }
        }
    
        let csv_content = String::from_utf8(wtr.into_inner()?)?;
        file.write_all(csv_content.as_bytes()).await?;
        println!("CSV file written successfully");
        Ok(())
    }

        pub async fn dump_all_pages_json(&self) -> Result<(), AppError> {
            let mut all_listings = Vec::new();
            for page in 1..=self.total_pages {
                let (listings, _) = fetch_and_parse_listings(&self.query, page).await?;
                all_listings.extend(listings);
            }
            
            // Write all_listings to a file
            // You can use serde to serialize the data to JSON or CSV
            // For example, using serde_json:
            let json = serde_json::to_string(&all_listings)?;
            std::fs::write("all_listings.json", json)?;
            
            Ok(())
        }

    pub async fn perform_search(&mut self) -> Result<(), AppError> {
        self.current_page = 1;
        let (listings, total_pages) = fetch_and_parse_listings(&self.query, self.current_page).await?;
        self.listings = listings;
        self.total_pages = total_pages;
        self.selected_index = 0;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), AppError> {
        // Setup terminal
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Fetch initial listings
        let (listings, total_pages) = fetch_and_parse_listings("https://www.olx.pl/elektronika/komputery/", 1).await?;
        self.listings = listings;
        self.total_pages = total_pages;
        // Main event loop
        loop {
            terminal.draw(|f| crate::ui::ui::draw(f, self))?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') if !self.show_dialog => break,
                    KeyCode::Char('d') if !self.show_dialog => self.toggle_dialog(),
                    KeyCode::Down => {
                        if !self.show_dialog && self.selected_index < self.listings.len().saturating_sub(1) {
                            self.selected_index += 1;
                        }
                    }
                    
                    KeyCode::Char('s') => {
                        if !self.show_dialog {
                            self.dump_all_pages_csv().await ?;
                        }
                    }


                    KeyCode::Char('x') => {
                        if !self.show_dialog {
                            self.dump_all_pages_json().await?;
                        }
                    }
                    


                    KeyCode::Up => {
                        if !self.show_dialog && self.selected_index > 0 {
                            self.selected_index -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if !self.show_dialog {
                            self.next_page().await?;
                        }
                    }
                    KeyCode::Left => {
                        if !self.show_dialog {
                            self.prev_page().await?;
                        }
                    }
                    KeyCode::Char(c) => {
                        if self.show_dialog {
                            self.input_char(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if self.show_dialog {
                            self.backspace();
                        }
                    }
                    KeyCode::Enter => {
                        if self.show_dialog {
                            self.toggle_dialog();
                            self.perform_search().await?;
                        }
                    }

                    KeyCode::Esc => {
                        if self.show_dialog {
                            self.toggle_dialog();
                        }
                    }
                    _ => {}
                }
            }
        }

        // Restore terminal
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen
        )?;

        Ok(())
    }
}




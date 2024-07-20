use crate::data::models::ListingData;
use crate::error::AppError;
use crate::scraper::olx::fetch_and_parse_listings;
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;


pub struct App {
    pub listings: Vec<ListingData>,
    pub selected_index: usize,
    pub show_dialog: bool,
    pub query: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            listings: Vec::new(),
            selected_index: 0,
            show_dialog: true,
            query: String::from("karta graficzna"),
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


    pub async fn perform_search(&mut self) -> Result<(), AppError> {
        self.listings = fetch_and_parse_listings(&self.query).await?;
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
        self.listings = fetch_and_parse_listings("https://www.olx.pl/elektronika/komputery/").await?;

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
                    KeyCode::Up => {
                        if !self.show_dialog && self.selected_index > 0 {
                            self.selected_index -= 1;
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




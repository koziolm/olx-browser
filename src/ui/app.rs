use crate::data::models::ListingData;
use crate::error::AppError;
use crate::scraper::olx;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};


#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    pub listings: Vec<ListingData>,
    pub selected_listing: usize,
    pub error: Option<String>,
    pub query: String,
    pub input: String,
    pub input_mode: InputMode,
}


impl App {
    pub fn new() -> Self {
        Self {
            listings: Vec::new(),
            selected_listing: 0,
            error: None,
            query: String::from("karta graficzna"),
            input: String::new(),
            input_mode: InputMode::Normal,
        }
    }



    pub fn next_listing(&mut self) {
        if !self.listings.is_empty() {
            self.selected_listing = (self.selected_listing + 1) % self.listings.len();
        }
    }

    pub fn previous_listing(&mut self) {
        if !self.listings.is_empty() {
            self.selected_listing = if self.selected_listing > 0 {
                self.selected_listing - 1
            } else {
                self.listings.len() - 1
            };
        }
    }

    pub fn add_char_to_input(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop_char_from_input(&mut self) {
        self.input.pop();
    }

    pub async fn run(&mut self) -> Result<(), AppError> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        self.fetch_listings().await?;

        loop {
            terminal.draw(|f| crate::ui::widgets::ui(f, self))?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('e') => {
                            self.input_mode = InputMode::Editing;
                            self.input = self.query.clone();
                        }
                        KeyCode::Down => {
                            if !self.listings.is_empty() {
                                self.selected_listing = (self.selected_listing + 1) % self.listings.len();
                            }
                        },
                        KeyCode::Up => {
                            if !self.listings.is_empty() {
                                self.selected_listing = if self.selected_listing > 0 {
                                    self.selected_listing - 1
                                } else {
                                    self.listings.len() - 1
                                };
                            }
                        },
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            self.query = self.input.clone();
                            self.input_mode = InputMode::Normal;
                            self.fetch_listings().await?;
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
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

    async fn fetch_listings(&mut self) -> Result<(), AppError> {
        let url = format!("https://www.olx.pl/oferty/q-{}/", self.query.replace(" ", "-"));
        match olx::fetch_and_parse_listings(&url).await {
            Ok(listings) => {
                self.listings = listings;
                self.error = None;
            }
            Err(e) => self.error = Some(format!("Error fetching listings: {}", e)),
        }
        Ok(())
    }
}

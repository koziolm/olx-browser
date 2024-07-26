use crate::ui::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState},
    Frame,
};

use ratatui::text::Text;


pub fn draw(f: &mut Frame, app: &App) {
    if app.show_dialog {
        draw_dialog(f, app);
    } else {
        draw_listings(f, app);
    }
}

fn draw_dialog(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area);
    
    let input = Paragraph::new(Text::from(app.query.clone()))
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Search Query"));
    
    f.render_widget(input, area);
}

fn draw_listings(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    if app.listings.is_empty() {
        let message = Paragraph::new(Text::from("No listings found. Press 'd' to search."))
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Listings"));
        f.render_widget(message, chunks[0]);
        return;
    }
    
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Title", "Price", "Location", "Condition"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);

    let rows = app.listings.iter().map(|item| {
        let cells = vec![
            Cell::from(item.title.clone()),
            Cell::from(item.price.clone()),
            Cell::from(item.location_date.clone()),
            Cell::from(item.condition.clone()),
        ];
        Row::new(cells)
    });

    let t = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Listings"))
    .highlight_style(selected_style)
    .highlight_symbol(">> ");

    let mut state = TableState::default();
    state.select(Some(app.selected_index));



        
    f.render_stateful_widget(&      t, chunks[0], &mut state);

    let page_info = Paragraph::new(format!("Page {} of {}", app.current_page, app.total_pages))
    .style(Style::default().fg(Color::Yellow))
    .alignment(Alignment::Center);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(chunks[0]);

    f.render_stateful_widget(t, chunks[0], &mut state);
    f.render_widget(page_info, chunks[1]);

    
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
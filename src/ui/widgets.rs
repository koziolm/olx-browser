use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Frame,
};
use super::app::{App, InputMode};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(3), Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let query_display = Paragraph::new(format!("Active Query: {}", app.query))
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL).title("Query"));
    f.render_widget(query_display, chunks[0]);

    if let Some(error) = &app.error {
        let error_msg = Paragraph::new(error.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title("Error"));
        f.render_widget(error_msg, chunks[1]);
    }

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[2]);

    let items: Vec<ListItem> = app.listings
        .iter()
        .enumerate()
        .map(|(i, listing)| {
            let content = vec![Spans::from(listing.title.clone())];
            if i == app.selected_listing {
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

    if !app.listings.is_empty() {
        let selected = &app.listings[app.selected_listing];
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

    if app.input_mode == InputMode::Editing {
        let input = Paragraph::new(app.input.as_ref())
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
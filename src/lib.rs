pub mod ui;
pub mod data;
pub mod scraper;
pub mod error;

use ui::app::App;

pub async fn run() -> Result<(), error::AppError> {
    let mut app = App::new();
    app.run().await
}
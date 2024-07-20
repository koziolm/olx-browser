pub mod ui;
pub mod data;
pub mod scraper;
pub mod error;

use ui::app::App;
use error::AppError;

pub async fn run() -> Result<(), AppError> {
    let mut app = App::new();
    app.run().await
}
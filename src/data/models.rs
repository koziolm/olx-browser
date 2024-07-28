use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]                               
pub struct ListingData {
    pub id: String,
    pub url: String,
    pub title: String,
    pub price: String,
    pub image_url: String,
    pub location_date: String,
    pub condition: String,
    pub is_featured: bool,
    pub has_delivery: bool,
    pub has_safety_badge: bool,
}
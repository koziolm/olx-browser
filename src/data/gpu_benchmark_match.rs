use csv::Reader;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::Deserialize;
use std::error::Error;
use std::cmp::Ordering;
use std::io::{self, Write};
use crate::data::models;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
struct Listing {
    #[serde(flatten)]
    data: models::ListingData,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
struct Benchmark {
    Type: String,
    #[serde(rename = "Part Number")]
    PartNumber: String,
    Brand: String,
    Model: String,
    Rank: String,
    Benchmark: String,
    Samples: String,
    URL: String,
}

#[derive(Debug, Clone)]
struct MatchResult {
    listing: Listing,
    benchmark: Benchmark,
    score: i64,
    ratio: f64,
}

fn parse_price(price: &str) -> f64 {
    price.replace(" ", "")
         .replace("zł", "")
         .replace(",", ".")  // Replace comma with dot for decimal point
         .parse::<f64>()
         .unwrap_or(0.0)
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


fn main() -> Result<(), Box<dyn Error>> {
    let min_price = get_user_input("Enter minimum price (in zł) or press Enter for no minimum: ");
    let max_price = get_user_input("Enter maximum price (in zł) or press Enter for no maximum: ");
    let condition = get_user_input("Enter preferred condition (New/Used) or press Enter for any: ");

    let min_price: Option<f64> = if min_price.is_empty() { None } else { Some(parse_price(&min_price)) };
    let max_price: Option<f64> = if max_price.is_empty() { None } else { Some(parse_price(&max_price)) };
    let condition = if condition.is_empty() { None } else { Some(condition.to_lowercase()) };

    let mut listings = Reader::from_path("listings.csv")?;
    let mut benchmarks = Reader::from_path("benchmarks.csv")?;

    let matcher = SkimMatcherV2::default();
    let mut all_matches: Vec<MatchResult> = Vec::new();

    for listing in listings.deserialize() {
        let listing: Listing = listing?;
        let price = parse_price(&listing.data.price);

        // Check if the listing matches user preferences
        if (min_price.is_none() || price >= min_price.unwrap()) &&
           (max_price.is_none() || price <= max_price.unwrap()) &&
           (condition.is_none() || listing.data.condition.to_lowercase() == *condition.as_ref().unwrap()) {
            let mut best_match: Option<MatchResult> = None;

            // Reset the CSV reader to the beginning for each listing
            benchmarks = Reader::from_path("benchmarks.csv")?;

            for benchmark in benchmarks.deserialize() {
                let benchmark: Benchmark = benchmark?;
                let score = matcher.fuzzy_match(&listing.data.title, &benchmark.Model);

                if let Some(score) = score {
                    let benchmark_value = benchmark.Benchmark.parse::<f64>().unwrap_or(0.0);
                    let ratio = if price > 0.0 { benchmark_value / price } else { 0.0 };

                    let match_result = MatchResult {
                        listing: listing.clone(),
                        benchmark: benchmark.clone(),
                        score,
                        ratio,
                    };

                    if best_match.is_none() || score > best_match.as_ref().unwrap().score {
                        best_match = Some(match_result);
                    }
                }
            }

            if let Some(match_result) = best_match {
                all_matches.push(match_result);
            }
        }
    }

        // Sort matches by ratio (descending order)
        all_matches.sort_by(|a, b| b.ratio.partial_cmp(&a.ratio).unwrap_or(Ordering::Equal));

        // Print top 10 offers
        println!("Top 10 offers by benchmark/price ratio:");
        for (i, match_result) in all_matches.iter().take(10).enumerate() {
            println!("{}. Listing: {}", i + 1, match_result.listing.data.title);
            println!("   Price: {}", match_result.listing.data.price);
            println!("   Condition: {}", match_result.listing.data.condition);
            println!("   Benchmark: {}", match_result.benchmark.Benchmark);
            println!("   Benchmark/Price Ratio: {:.6}", match_result.ratio);
            println!("   Listing URL: {}", match_result.listing.data.url);
            println!("   Benchmark URL: {}", match_result.benchmark.URL);
            println!("---");
        }   
    
        Ok(())
    }

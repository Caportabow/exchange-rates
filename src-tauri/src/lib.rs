use reqwest::Client;
use select::document::Document;
use select::predicate::{Class, Name};
use regex::Regex;
use chrono::{Datelike, Local, NaiveDate};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use anyhow::Context;
use futures::future::join_all;
use std::sync::Arc;

#[derive(Serialize)]
struct CurrencyRate {
    from: String,
    to: String,
    rate: f64,
    mtd: f64,
    ytd: f64,
}

#[derive(Deserialize)]
struct ApiResponse {
    rates: std::collections::HashMap<String, f64>,
}

async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let res = client.get(url).send().await?.text().await?;
    Ok(res)
}

// Extracts first floating-point number from a string
fn extract_first_number(text: &str) -> Option<f64> {
    let re = Regex::new(r"\d+\.\d+").unwrap();
    re.find(text).and_then(|m| m.as_str().parse::<f64>().ok())
}

async fn fetch_rate(target_currency: &str, currency: &str) -> Result<f64> {
    let base_url = format!("https://open.er-api.com/v6/latest/{}", target_currency);
    
    let response = Client::new()
        .get(&base_url)
        .send()
        .await
        .context("Failed to send request")?;
    
    let text = response.text().await.context("Failed to read response text")?;
    let response_json: ApiResponse = serde_json::from_str(&text).context("Failed to parse JSON")?;

    // Convert currency to uppercase before lookup
    let rate = response_json.rates.get(&currency.to_uppercase()).copied().unwrap_or(0.0);

    Ok(rate)
}

// Function to extract average values from the HTML response
fn extract_average_values(html: &str, month: &str, target_date: &str) -> (f64, f64, f64) {
    let document = Document::from(html);

    // Extract YTD average
    let ytd_avg = document.find(Class("history-rate-summary"))
        .next()
        .and_then(|table| {
            table.find(Name("tr"))
                .filter(|row| row.text().contains("Average:"))
                .next()
                .and_then(|row| row.find(Name("td")).nth(1))
                .and_then(|cell| extract_first_number(&cell.text()))
        })
        .unwrap_or(0.0);

    // Extract MTD average for the specified month
    let mtd_avg = document.find(Class("month-footer"))
        .filter(|td| td.text().to_lowercase().contains(&month.to_lowercase())) // filter by month
        .next()
        .and_then(|td| {
            // Collect all spans with class "nowrap" in the found month-footer
            let found_spans: Vec<_> = td.find(Class("nowrap"))
                .map(|span| span.text())
                .collect();

            // Find the span that corresponds to "Average" (last in the list)
            found_spans.last()
                .and_then(|rate_text| {
                    // Extract the number from the rate text (e.g., "¥1 = €0.1289")
                    extract_first_number(rate_text)
                })
        })
        .unwrap_or(0.0);

    // Extract rate for the specified date
    let mut rate = 0.0;
    let table = document.find(Class("history-rates-data")).next();
    if let Some(table) = table {
        // Iterate over each <tr> element in the table
        for tr in table.find(Name("tr")) {
            // Find the <a> tag containing the date in YYYY-MM-DD format
            if let Some(date_element) = tr.find(Class("n")).next() {
                let date = date_element.text().trim().to_string();
                // If the date matches the target date, extract the exchange rate
                if date == target_date {
                    // Extract exchange rate from <span> inside <td>
                    if let Some(rate_element) = tr.find(Name("span")).next() {
                        let rate_text = rate_element.text().trim().to_string();
                        // Extract the numerical exchange rate part
                        let rate_parts: Vec<&str> = rate_text.split_whitespace().collect();
                        if rate_parts.len() == 5 {
                            // Convert the exchange rate string to a floating-point value
                            if let Ok(rate_value) = rate_parts[3].parse::<f64>() {
                                rate = rate_value;
                            }
                        }
                    }
                    break;  // Break after finding the correct date
                }
            }
        }
    }

    (ytd_avg, mtd_avg, rate)
}

#[tauri::command]
async fn parser(target_currency: String, date: Option<String>) -> Result<Vec<CurrencyRate>, String> {
    let date = date
        .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok())
        .unwrap_or_else(|| Local::now().date_naive());
    let year = date.year();
    let current_month = date.month();
    let month = NaiveDate::from_ymd_opt(year, current_month, 1)
        .expect("Invalid date")
        .format("%B")
        .to_string();

    // Arc is now shared and cloned within the async task
    let currencies = Arc::new(vec!["usd", "eur", "gbp", "jpy", "chf", "cny"]);

    let mut tasks: Vec<tokio::task::JoinHandle<Result<Option<CurrencyRate>, String>>> = Vec::new();

    // Use `Arc::clone` to share the data with async tasks
    for currency in currencies.iter().filter(|&&currency| currency != target_currency) {
        let exchange = format!("{}-{}", target_currency.to_string(), currency);
        let url = format!("https://www.exchange-rates.org/exchange-rate-history/{}-{}", exchange, year);
        let target_currency_clone = target_currency.clone(); // Clone target_currency to use inside async block
        let month_clone = month.clone(); // Clone month to use inside async block
        let date_clone = date.format("%Y-%-m-%-d").to_string(); // Clone date to use inside async block

        // Spawn an async task for each currency exchange rate fetch
        let task = tokio::spawn({
            let currency = currency.to_string(); // Move `currency` to the async block
            async move {
                match fetch_html(&url).await {
                    Ok(html) => {
                        let (ytd_avg, mtd_avg, mut rate) = extract_average_values(&html, &month_clone, &date_clone);
                        if rate == 0.0 {
                            rate = fetch_rate(&target_currency_clone, &currency)
                                                .await
                                                .expect("Failed to fetch exchange rate");
                        };
                        Ok(Some(CurrencyRate {
                            from: target_currency_clone.to_string(),
                            to: currency,
                            rate: rate,
                            mtd: mtd_avg,
                            ytd: ytd_avg,
                        }))
                    }
                    Err(e) => {
                        println!("Error fetching HTML for {}: {}", currency, e);
                        Ok(None)
                    }
                }
            }
        });

        tasks.push(task);
    }

    // Await all tasks concurrently
    let results: Vec<CurrencyRate> = join_all(tasks)
        .await
        .into_iter()
        .filter_map(|res| {
            res.ok().and_then(|inner_res| inner_res.ok()).flatten()
        })
        .collect();

    if results.is_empty() {
        Err("No results found.".to_string())
    } else {
        Ok(results)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![parser])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

use reqwest;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;
use std::error::Error;
use html2md;
use crate::utils::token_counter::token_count_4o;

const BASE_URL: &str = "https://developers.binance.com/docs/binance-spot-api-docs";

const ENDPOINTS: &[&str] = &[
    "filters",
    "enums",
    "rest-api/general-api-information",
    "rest-api/http-return-codes",
    "rest-api/error-codes",
    "rest-api/general-information-on-endpoints",
    "rest-api/limits",
    "rest-api/data-sources",
    "rest-api/endpoint-security-type",
    "rest-api/general-endpoints",
    "rest-api/market-data-endpoints",
    "rest-api/trading-endpoints",
    "rest-api/account-endpoints",
    "user-data-stream",
    "rest-api/user-data-stream-endpoints",
    "errors",
];

const OUTPUT_FILE: &str = "binance_spot_rest_api_docs.md";

pub async fn process_docs() -> Result<(), Box<dyn Error>> {
    println!("Downloading Binance API Documentation HTML files...");
    
    let mut all_markdown = String::from("# Binance API Documentation\n\n");
    
    for &endpoint in ENDPOINTS {
        let full_url = format!("{}/{}", BASE_URL, endpoint);
        println!("Processing: {}", full_url);
        if let Ok(html) = fetch_page(&full_url).await {
            let title = endpoint
                .split('/')
                .last()
                .unwrap_or("")
                .replace(".html", "")
                .replace("-", " ")
                .to_uppercase();
            all_markdown.push_str(&format!("## {}\n\n", title));
            all_markdown.push_str(&extract_content(&html));
            all_markdown.push_str("\n\n");
        }
    }
    
    let mut file = File::create(OUTPUT_FILE)?;
    file.write_all(all_markdown.as_bytes())?;
    
    println!("Documentation saved to {}", OUTPUT_FILE);
    token_count_4o();
    Ok(())
}

async fn fetch_page(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?;
    Ok(response)
}

fn extract_content(html: &str) -> String {
    let document = Html::parse_document(html);
    let selector = Selector::parse("div.theme-doc-markdown.markdown").unwrap();
    if let Some(element) = document.select(&selector).next() {
        let html_content = element.inner_html();
        html2md::parse_html(&html_content) // Convert HTML to Markdown
    } else {
        String::from("Content not found.")
    }
}

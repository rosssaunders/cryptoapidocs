use reqwest;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;
use std::error::Error;
use html2md;
use crate::utils::token_counter::token_count_4o;
use chrono::Utc;

const BASE_URL: &str = "https://bybit-exchange.github.io/docs/v5";

pub struct DocProcessor {
    endpoints: &'static [&'static str],
    output_file: &'static str,
    title: &'static str,
}

impl DocProcessor {
    pub fn new(endpoints: &'static [&'static str], output_file: &'static str, title: &'static str) -> Self {
        Self {
            endpoints,
            output_file,
            title,
        }
    }

    pub async fn process_docs(&self) -> Result<(u32, String, String), Box<dyn Error>> {
        println!("Downloading ByBit API Documentation HTML files...");
        
        let mut all_markdown = String::from(format!("# {}\n\n", self.title));
        for &endpoint in self.endpoints {

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
        
        let output_path = format!("docs/{}", self.output_file);
        let dir_path = std::path::Path::new(&output_path).parent().unwrap_or_else(|| std::path::Path::new(""));

        std::fs::create_dir_all(&dir_path)?;
        let mut file = File::create(&output_path)?;
        file.write_all(all_markdown.as_bytes())?;
        
        println!("Documentation saved to {}", output_path);
        let token_count = token_count_4o(&output_path);
        let timestamp = Utc::now().to_rfc3339();
        let market_name = self.title.to_string();
        
        Ok((token_count, timestamp, market_name))
    }
}

async fn fetch_page(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?;
    if response.contains("Page Not Found") {
        panic!("Page not found error encountered for URL: {}", url);
    }
    Ok(response)
}

fn extract_content(html: &str) -> String {
    let document = Html::parse_document(html);
    let selector = Selector::parse(".theme-doc-markdown.theme-api-markdown.markdown").unwrap();
    if let Some(element) = document.select(&selector).next() {
        let html_content = element.inner_html();
        html2md::parse_html(&html_content) // Convert HTML to Markdown
    } else {
        String::from("Content not found.")
    }
}
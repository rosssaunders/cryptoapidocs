use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Section {
    pub h1_match: String,
    pub h2_match: String,
    pub h3_matches: Vec<String>,
    pub include_h2_html: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSection {
    pub sections: Vec<Section>,
    pub output_file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configs {
    pub url: String,
    pub configs: HashMap<String, ConfigSection>,
}

impl Configs {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let configs: Configs = serde_json::from_str(&content)?;
        Ok(configs)
    }

    pub fn save_example(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let example = Configs {
            url: "https://www.okx.com/docs-v5/en/#overview".to_string(),
            configs: {
                let mut map = HashMap::new();
                
                // Example 1: Account and Trading endpoints
                map.insert("account_and_trading".to_string(), ConfigSection {
                    sections: vec![
                        Section {
                            h1_match: "account".to_string(),
                            h2_match: "funding".to_string(),
                            h3_matches: vec!["post".to_string(), "get".to_string()],
                            include_h2_html: false,
                        },
                        Section {
                            h1_match: "order-book-trading".to_string(),
                            h2_match: "trade".to_string(),
                            h3_matches: vec!["post".to_string(), "get".to_string()],
                            include_h2_html: false,
                        }
                    ],
                    output_file: "account_and_trading_api.md".to_string(),
                });

                // Example 2: All trading related endpoints
                map.insert("all_trading".to_string(), ConfigSection {
                    sections: vec![
                        Section {
                            h1_match: "order-book-trading".to_string(),
                            h2_match: "trade".to_string(),
                            h3_matches: vec!["post".to_string(), "get".to_string()],
                            include_h2_html: false,
                        },
                        Section {
                            h1_match: "algo-trading".to_string(),
                            h2_match: "trade".to_string(),
                            h3_matches: vec!["post".to_string(), "get".to_string()],
                            include_h2_html: false,
                        },
                        Section {
                            h1_match: "block-trading".to_string(),
                            h2_match: "orders".to_string(),
                            h3_matches: vec!["post".to_string(), "get".to_string()],
                            include_h2_html: false,
                        }
                    ],
                    output_file: "all_trading_api.md".to_string(),
                });

                map
            },
        };
        let content = serde_json::to_string_pretty(&example)?;
        fs::write(path, content)?;
        Ok(())
    }
} 
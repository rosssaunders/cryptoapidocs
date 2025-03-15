use std::error::Error;
use std::fs;
use tiktoken_rs::o200k_base;
use crate::exchanges::processor_registry;

pub fn count_tokens_in_file(filepath: &str) -> Result<u32, Box<dyn Error>> {
    let content = fs::read_to_string(filepath)?;
    let bpe = o200k_base()?;
    let tokens = bpe.encode_with_special_tokens(&content);
    Ok(tokens.len() as u32)
}

pub fn process_all_docs() -> Result<(), Box<dyn Error>> {
    // Print the table header
    println!("\n| Exchange | API Type | Documentation | Last Updated | Token Count |");
    println!("|----------|-----------|---------------|--------------|-------------|");
    
    // Get all exchanges
    let exchanges = ["binancespot", "binanceusdm", "binancecoinm"];
    
    for exchange in exchanges {
        let processors = processor_registry::create_processors_by_exchange(exchange);
        
        for processor in processors {
            let filepath = format!("docs/{}", processor.get_output_filename());
            match count_tokens_in_file(&filepath) {
                Ok(token_count) => {
                    // Extract exchange and API type from the path
                    let parts: Vec<&str> = filepath.split('/').collect();
                    let exchange = parts.get(1).unwrap_or(&"Unknown").to_string();
                    let api_type = parts.get(2).unwrap_or(&"Unknown").to_string();
                    let doc_name = parts.last().unwrap_or(&"Unknown").to_string();
                    
                    println!("| {} | {} | {} | {} | {} |",
                        exchange.to_string().to_uppercase(),
                        api_type,
                        doc_name,
                        chrono::Utc::now().format("%Y-%m-%d"),
                        token_count);
                }
                Err(e) => {
                    eprintln!("Error processing {}: {}", filepath, e);
                }
            }
        }
    }
    
    Ok(())
} 
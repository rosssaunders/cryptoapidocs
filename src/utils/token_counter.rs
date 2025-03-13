use std::fs;

pub fn token_count_4o() {
    // Step 1: Load the Markdown file
    let filepath = "/Users/rosssaunders/DevDrive/cryptoapidocs/binance_spot_rest_api_docs.md";
    let content = fs::read_to_string(filepath).expect("Failed to read the file");

    use tiktoken_rs::o200k_base;

    let bpe = o200k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(
        &content
    );
    println!("Token count: {}", tokens.len());
}

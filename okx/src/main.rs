mod config;

use kuchiki::traits::*;
use std::error::Error;
use html2md::parse_html;
use std::io::Write;
use std::path::Path;
use config::{Section, ConfigSection, Configs};

fn extract_sections(document: &kuchiki::NodeRef, section: &Section) -> Vec<(String, String)> {
    let mut extracted_sections = Vec::new();
    
    // Process H1 sections
    for h1_node in document.select("h1").unwrap() {
        let h1_ref = h1_node.as_node();
        if let Some(element) = h1_ref.as_element() {
            if let Some(id) = element.attributes.borrow().get("id") {
                if !id.contains(&section.h1_match) {
                    continue;
                }

                println!("Found H1 section: {}", id);
                
                // Process H2 sections within matching H1
                let mut sibling = h1_ref.next_sibling();
                while let Some(sib) = sibling {
                    if let Ok(h2_nodes) = sib.select("h2") {
                        for h2_node in h2_nodes {
                            let h2_ref = h2_node.as_node();
                            if let Some(h2_element) = h2_ref.as_element() {
                                if let Some(h2_id) = h2_element.attributes.borrow().get("id") {
                                    if !h2_id.contains(&section.h2_match) {
                                        continue;
                                    }

                                    println!("Found H2 section: {}", h2_id);

                                    if section.include_h2_html {
                                        let section_html = extract_section_content(h2_ref);
                                        extracted_sections.push((h2_id.to_string(), section_html));
                                    }
                                    
                                    // Process H3 sections within matching H2
                                    let mut h2_sibling = h2_ref.next_sibling();
                                    while let Some(h2_sib) = h2_sibling {
                                        if let Ok(h3_nodes) = h2_sib.select("h3") {
                                            for h3_node in h3_nodes {
                                                let h3_ref = h3_node.as_node();
                                                if let Some(h3_element) = h3_ref.as_element() {
                                                    if let Some(h3_id) = h3_element.attributes.borrow().get("id") {
                                                        if section.h3_matches.iter().any(|pattern| h3_id.contains(pattern)) {
                                                            let section = extract_section_content(h3_ref);
                                                            extracted_sections.push((h3_id.to_string(), section));

                                                            println!("Found H3 section: {}", h3_id);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        h2_sibling = h2_sib.next_sibling();
                                    }
                                }
                            }
                        }
                    }
                    sibling = sib.next_sibling();
                }
            }
        }
    }
    
    extracted_sections
}

fn extract_section_content(node: &kuchiki::NodeRef) -> String {
    let mut section_html = String::new();
    section_html.push_str(&node.to_string());

    let mut sibling = node.next_sibling();
    while let Some(sib) = sibling {
        if let Some(element_data) = sib.as_element() {
            let tag_name = element_data.name.local.as_ref();
            if tag_name.starts_with('h') {
                if let Ok(level) = tag_name[1..].parse::<u8>() {
                    if level <= 3 {
                        break;
                    }
                }
            }
        }
        section_html.push_str(&sib.to_string());
        sibling = sib.next_sibling();
    }
    
    section_html
}

fn process_config(document: &kuchiki::NodeRef, name: &str, config: &ConfigSection) -> Result<(), Box<dyn Error>> {
    println!("Processing configuration: {}", name);
    
    // Extract sections based on each section configuration
    let mut all_extracted_sections = Vec::new();
    for section in &config.sections {
        let extracted = extract_sections(document, section);
        all_extracted_sections.extend(extracted);
    }

    if all_extracted_sections.is_empty() {
        println!("Warning: No sections found for configuration '{}'", name);
        return Ok(());
    }

    // Create a combined markdown string
    let mut combined_markdown = String::from(&format!("# OKX API Documentation - {}\n\n", name));

    // Convert each section to markdown and save
    for (id, html) in all_extracted_sections {
        let markdown = parse_html(&html);
        
        // Add section to combined markdown with a separator
        //combined_markdown.push_str(&format!("\n## {}\n\n", id));
        combined_markdown.push_str(&markdown);
        combined_markdown.push_str("\n\n---\n\n");
        
        //println!("Processed section: {}", id);
    }

    // Save the combined markdown to the configured output file
    let mut combined_file = std::fs::File::create(&config.output_file)?;
    combined_file.write_all(combined_markdown.as_bytes())?;
    println!("Created documentation file: {}", config.output_file);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new("config.json");
    
    // If config doesn't exist, create an example one
    if !config_path.exists() {
        println!("Creating example configuration file...");
        Configs::save_example(config_path)?;
        println!("Example configuration saved to config.json");
        println!("Please modify the configuration file and run the program again.");
        return Ok(());
    }

    // Load configurations
    let configs = Configs::load(config_path)?;
    
    // Fetch the HTML page
    println!("Fetching documentation from {}", configs.url);
    let resp = reqwest::blocking::get(&configs.url)?.text()?;

    // Save the html to a file for reference
    let mut file = std::fs::File::create("okx.html")?;
    file.write_all(resp.as_bytes())?;
    println!("Saved raw HTML to okx.html");
    
    // Parse the HTML document using kuchiki
    let document = kuchiki::parse_html().one(resp);

    // Process each configuration
    for (name, config) in configs.configs {
        if let Err(e) = process_config(&document, &name, &config) {
            eprintln!("Error processing configuration '{}': {}", name, e);
        }
    }

    Ok(())
}
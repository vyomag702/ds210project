// src/main.rs
mod loader;
mod analyzer;

use std::process;
use crate::loader::ProductDataset;
use crate::analyzer::ProductAnalyzer;

#[derive(Debug, Clone)]
pub struct Product {
    pub asin: String,
    pub title: String,
    pub group: String,
    pub salesrank: i32,
}

fn main() {
    println!("Amazon Product Trend Analyzer");
    println!("----------------------------");

    // Load dataset
    let dataset = match load_dataset() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Fatal error during data loading: {}", e);
            process::exit(1);
        }
    };

    // Initialize analyzer
    let analyzer = ProductAnalyzer::new(&dataset);


    analyze_bestsellers(&analyzer);
    analyze_trends(&analyzer);
    analyze_competition(&analyzer);
}

fn load_dataset() -> Result<ProductDataset, Box<dyn std::error::Error>> {
    const DATA_PATH: &str = "data/amazon-meta.txt";
    
    println!("[1/3] Loading dataset from {}...", DATA_PATH);
    let dataset = loader::load_dataset(DATA_PATH)?;
    
    println!("Loaded {} products with {} connections", 
        dataset.products.len(), 
        dataset.graph.edge_count());
    
    Ok(dataset)
}

/// Question A: Identify bestsellers
fn analyze_bestsellers(analyzer: &ProductAnalyzer) {
    println!("\n[2/3] Identifying top products...");
    
    let top_products = analyzer.top_products_by_connections(5);
    
    println!("\nTop 5 Products by Market Connections:");
    for (i, product) in top_products.iter().enumerate() {
        println!("{}. {}", i + 1, product.title);
        println!("   - ASIN: {}", product.asin);
        println!("   - Category: {}", product.group);
        println!("   - Sales Rank: {}", product.salesrank);
    }
}

/// Question B: Detect trend clusters
fn analyze_trends(analyzer: &ProductAnalyzer) {
    println!("\n[3/3] Detecting product trends...");
    
    let trends = analyzer.detect_trend_clusters();
    
    if trends.is_empty() {
        println!("Trend analysis not yet implemented");
    } else {
        // Will be populated in next commit
    }
}

/// Question C: Find low-competition products
fn analyze_competition(analyzer: &ProductAnalyzer) {
    let emerging = analyzer.find_low_competition_products();
    
    if emerging.is_empty() {
        println!("\n Competition analysis not yet implemented");
    } else {
        // Will be populated in next commit
    }
}
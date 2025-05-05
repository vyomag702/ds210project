// Main module for Amazon Product Trend Analyzer
mod loader;
mod analyzer;

use std::process;
use crate::loader::ProductDataset;
use crate::analyzer::ProductAnalyzer;

// Main function
fn main() {
    println!("Amazon Product Trend Analyzer");
    println!("----------------------------");
    
    // Loading dataset
    let dataset = match load_dataset() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Fatal error: {}", e);
            process::exit(1);
        }
    };
    
    // Create analyzer and print stats
    let analyzer = ProductAnalyzer::new(&dataset);
    analyzer.print_stats();

    // Run analyses
    analyze_bestsellers(&analyzer);
    analyze_trends(&analyzer);
    analyze_competition(&analyzer);
}

// Loads dataset from default path
fn load_dataset() -> Result<ProductDataset, Box<dyn std::error::Error>> {
    const DATA_PATH: &str = "data/amazon-meta.txt";
    println!("[1/3] Loading dataset...");
    loader::load_dataset(DATA_PATH)
}

// Analyzes and prints top products by connections
fn analyze_bestsellers(analyzer: &ProductAnalyzer) {
    println!("\n[2/3] Identifying top products...");
    let top_products = analyzer.top_products_by_connections(5);

    if top_products.is_empty() {
        println!("No products found with connections.");
        return;
    }

    println!("\nTop 5 Best-Selling Products:");
    for (i, product) in top_products.iter().enumerate() {
        println!("{}. {}", i + 1, product.title);
        println!("   - ASIN: {}", product.asin);
        println!("   - Category: {}", product.group);
        println!("   - Sales Rank: {}", product.salesrank);
    }
}

// Analyzes and prints product trend clusters
fn analyze_trends(analyzer: &ProductAnalyzer) {
    println!("\n[3/3] Detecting market trends...");
    let clusters = analyzer.detect_trend_clusters(5);

    if clusters.is_empty() {
        println!("No trend clusters found.");
        return;
    }

    println!("\nEmerging Product Trends:");
    for (i, cluster) in clusters.iter().take(3).enumerate() {
        println!("Trend Group {} ({} products):", i + 1, cluster.len());
        println!("Sample Products:");
        for product in cluster.iter().take(3) {
            println!("- {} (Rank: {})", product.title, product.salesrank);
        }
    }
}

// Analyzes and prints low competition products
fn analyze_competition(analyzer: &ProductAnalyzer) {
    println!("\n[4/4] Analyzing competition...");
    let opportunities = analyzer.find_low_competition_products(5);

    if opportunities.is_empty() {
        println!("No low competition products found.");
        return;
    }

    println!("\nBest Market Opportunities:");
    for (i, (product, score)) in opportunities.iter().enumerate() {
        println!("{}. {} (Opportunity Score: {:.2})", i + 1, product.title, score);
        println!("   - Current Rank: {}", product.salesrank);
        println!("   - Category: {}", product.group);
    }
}
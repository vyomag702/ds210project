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
    // loading the dataset
    let dataset = match load_dataset() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Fatal error: {}", e);
            process::exit(1);
        }
    };
    let analyzer = ProductAnalyzer::new(&dataset);
    analyzer.print_stats();  // Optional: Show graph metrics

    analyze_bestsellers(&analyzer);
    analyze_trends(&analyzer);
    analyze_competition(&analyzer);
}

fn load_dataset() -> Result<ProductDataset, Box<dyn std::error::Error>> {
    const DATA_PATH: &str = "data/amazon-meta.txt";
    println!("[1/3] Loading dataset...");
    loader::load_dataset(DATA_PATH)
}

fn analyze_bestsellers(analyzer: &ProductAnalyzer) {
    println!("\n[2/3] Identifying top products...");
    let top_products = analyzer.top_products_by_connections(5);

    println!("\nTop 5 Best-Selling Products:");
    for (i, product) in top_products.iter().enumerate() {
        println!("{}. {}", i + 1, product.title);
        println!("   - ASIN: {}", product.asin);
        println!("   - Category: {}", product.group);
        println!("   - Sales Rank: {}", product.salesrank);
    }
}

fn analyze_trends(analyzer: &ProductAnalyzer) {
    println!("\n[3/3] Detecting market trends...");
    let clusters = analyzer.detect_trend_clusters(5);  // Minimum cluster size

    println!("\nEmerging Product Trends:");
    for (i, cluster) in clusters.iter().take(3).enumerate() {
        println!("Trend Group {} ({} products):", i + 1, cluster.len());
        println!("Sample Products:");
        for product in cluster.iter().take(3) {
            println!("- {} (Rank: {})", product.title, product.salesrank);
        }
    }
}

fn analyze_competition(analyzer: &ProductAnalyzer) {
    println!("\n[4/4] Analyzing competition...");
    let opportunities = analyzer.find_low_competition_products(5);

    println!("\nBest Market Opportunities:");
    for (i, (product, score)) in opportunities.iter().enumerate() {
        println!("{}. {} (Opportunity Score: {:.2})", i + 1, product.title, score);
        println!("   - Current Rank: {}", product.salesrank);
        println!("   - Category: {}", product.group);
    }
}

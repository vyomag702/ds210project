// setting up the project
use petgraph::graph::Graph;
use std::collections::HashMap;

// created the product struct 
#[derive(Debug, Clone)]
struct Product {
    asin: String,
    title: String,
    group: String,
    salesrank: i32,
}

fn main() {
    println!("Amazon Product Trend Analyzer");
}
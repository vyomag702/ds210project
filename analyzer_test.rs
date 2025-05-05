use amazon_trends::{loader::{Product, ProductDataset}, analyzer::ProductAnalyzer};
use petgraph::graph::Graph;
use std::collections::HashMap;

#[test]
fn test_top_products_by_connections() {
    let dataset = create_connected_test_data();
    let analyzer = ProductAnalyzer::new(&dataset);
    
    let top = analyzer.top_products_by_connections(2);
    assert_eq!(top.len(), 2);
    assert_eq!(top[0].asin, "PROD1"); // Most connected (2 edges)
    assert_eq!(top[1].asin, "PROD2"); // Second most (1 edge)
}

#[test]
fn test_find_low_competition_products() {
    let dataset = create_opportunity_test_data();
    let analyzer = ProductAnalyzer::new(&dataset);
    
    let opportunities = analyzer.find_low_competition_products(1);
    assert_eq!(opportunities.len(), 1);
    assert_eq!(opportunities[0].0.asin, "PROD1"); // Best opportunity
}

#[test]
fn test_empty_dataset() {
    let dataset = ProductDataset {
        graph: Graph::new(),
        products: HashMap::new(),
    };
    let analyzer = ProductAnalyzer::new(&dataset);
    
    assert!(analyzer.top_products_by_connections(5).is_empty());
    assert!(analyzer.detect_trend_clusters(2).is_empty());
    assert!(analyzer.find_low_competition_products(5).is_empty());
}

// Helper functions
fn create_connected_test_data() -> ProductDataset {
    let mut graph = Graph::new();
    let mut products = HashMap::new();
    
    let node1 = graph.add_node("PROD1".to_string());
    let node2 = graph.add_node("PROD2".to_string());
    let node3 = graph.add_node("PROD3".to_string());
    
    // PROD1 has 2 connections (most)
    graph.add_edge(node1, node2, ());
    graph.add_edge(node1, node3, ());
    
    // PROD2 has 1 connection
    graph.add_edge(node2, node3, ());
    
    products.insert("PROD1".to_string(), product("PROD1", 1000));
    products.insert("PROD2".to_string(), product("PROD2", 2000));
    products.insert("PROD3".to_string(), product("PROD3", 500));
    
    ProductDataset { graph, products }
}

fn create_clustered_test_data() -> ProductDataset {
    let mut graph = Graph::new();
    let mut products = HashMap::new();
    
    let node1 = graph.add_node("PROD1".to_string());
    let node2 = graph.add_node("PROD2".to_string());
    let node3 = graph.add_node("PROD3".to_string());
    
    // Fully connected cluster of 3
    graph.add_edge(node1, node2, ());
    graph.add_edge(node2, node3, ());
    graph.add_edge(node1, node3, ());
    
    products.insert("PROD1".to_string(), product("PROD1", 1000));
    products.insert("PROD2".to_string(), product("PROD2", 2000));
    products.insert("PROD3".to_string(), product("PROD3", 1500));
    
    ProductDataset { graph, products }
}

fn create_opportunity_test_data() -> ProductDataset {
    let mut graph = Graph::new();
    let mut products = HashMap::new();
    
    let node1 = graph.add_node("PROD1".to_string());
    let node2 = graph.add_node("PROD2".to_string());
    
    // PROD1 has better rank and same connections as PROD2
    graph.add_edge(node1, node2, ());
    
    products.insert("PROD1".to_string(), product("PROD1", 500));  // Better rank
    products.insert("PROD2".to_string(), product("PROD2", 1000));
    
    ProductDataset { graph, products }
}

fn product(asin: &str, salesrank: i32) -> Product {
    Product {
        asin: asin.to_string(),
        title: format!("Test {}", asin),
        group: "Electronics".to_string(),
        salesrank,
    }
}
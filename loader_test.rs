use amazon_trends::loader::{load_dataset, Product, ProductDataset};
use std::io::Write;
use tempfile::NamedTempFile;
use petgraph::graph::Graph;
use std::collections::HashMap;

#[test]
fn test_load_empty_file() {
    let file = NamedTempFile::new().unwrap();
    assert!(load_dataset(file.path().to_str().unwrap()).is_err());
}

#[test]
fn test_load_malformed_data() {
    // Create badly formatted data (missing ASIN)
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "title: Broken Product").unwrap();
    file.flush().unwrap();
    
    assert!(load_dataset(file.path().to_str().unwrap()).is_err());
}

#[test]
fn test_load_dataset_with_similar_products() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "ASIN: TEST1\n title: Test 1\n group: Book\n salesrank: 100\n similar: 2 TEST2 TEST3").unwrap();
    writeln!(file, "\nASIN: TEST2\n title: Test 2\n group: Book").unwrap();
    writeln!(file, "\nASIN: TEST3\n title: Test 3\n group: Book").unwrap();
    file.flush().unwrap();
    
    let dataset = load_dataset(file.path().to_str().unwrap()).unwrap();
    assert_eq!(dataset.graph.edge_count(), 2);
}

// Helper function for other tests
pub fn create_test_dataset() -> ProductDataset {
    let mut graph = Graph::new();
    let mut products = HashMap::new();
    
    let node1 = graph.add_node("TEST1".to_string());
    let node2 = graph.add_node("TEST2".to_string());
    graph.add_edge(node1, node2, ());
    
    products.insert("TEST1".to_string(), 
        Product {
            asin: "TEST1".to_string(),
            title: "Test Product 1".to_string(),
            group: "Book".to_string(),
            salesrank: 100,
        }
    );
    
    products.insert("TEST2".to_string(), 
        Product {
            asin: "TEST2".to_string(),
            title: "Test Product 2".to_string(),
            group: "Book".to_string(),
            salesrank: 200,
        }
    );
    
    ProductDataset { graph, products }
}
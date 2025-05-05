// Module for analyzing product trends and relationships

use petgraph::algo::kosaraju_scc;
use petgraph::graph::Graph;
use std::collections::HashMap;

use crate::loader::{Product, ProductDataset};

// This struct is used to analyze product dataset relationships providing various insights
pub struct ProductAnalyzer<'a> {
    graph: &'a Graph<String, ()>,          // Refers to product graph
    products: &'a HashMap<String, Product>, // Refers to products map
}
impl<'a> ProductAnalyzer<'a> {
    // Creates new analyzer for given dataset
    pub fn new(dataset: &'a ProductDataset) -> Self {
        Self {
            graph: &dataset.graph,
            products: &dataset.products,
        }
    }
/*
    Finds top products by number of connections
    Arguments:
    'limit' - Maximum number of products to return
    Returns:
    Vector of product references sorted by connection count
*/
    pub fn top_products_by_connections(&self, limit: usize) -> Vec<&'a Product> {
        let mut products: Vec<_> = self.graph.node_indices()
            .filter_map(|node| {
                let asin = &self.graph[node];
                self.products.get(asin).map(|p| {
                    let connections = self.graph.neighbors(node).count();
                    (connections, p)
                })
            })
            .collect();
        // Sort by connection count 
        products.sort_by(|(a_conn, _), (b_conn, _)| b_conn.cmp(a_conn));
        // Return top N products        
        products.into_iter()
            .take(limit)
            .map(|(_, p)| p)
            .collect()
    }
/*
    Detects product trend clusters using strongly connected components
    arguments:
    'min_size' - Minimum cluster size to include
    Returns
    Vector of product clusters   
*/
    pub fn detect_trend_clusters(&self, min_size: usize) -> Vec<Vec<&'a Product>> {
        kosaraju_scc(self.graph)
            .into_iter()
            .filter(|c| c.len() >= min_size) // Filter small clusters
            .map(|cluster| {
                // Convert node indices to products
                cluster.iter()
                    .filter_map(|n| self.products.get(&self.graph[*n]))
                    .collect()
            })
            .filter(|c: &Vec<_>| !c.is_empty()) // Remove Empty Clusters
            .collect()
    }
    /*
    Finds products with low competition based on sales rank and cluster size
    Arguments:
    'top_n' - Number of products to return
    Returns:
    Vector of (product, score) tuples sorted by opportunity score
    */
    pub fn find_low_competition_products(&self, top_n: usize) -> Vec<(&'a Product, f32)> {
        let mut scores: Vec<(&Product, f32)> = self.graph.node_indices()
            .filter_map(|node| {
                let asin = &self.graph[node];
                let product = self.products.get(asin)?;
                
                // Filter out invalid sales ranks
                if product.salesrank <= 0 || product.salesrank > 100_000 {
                    return None;
                }

                // Calculate opportunity score
                let cluster_size = self.graph.neighbors(node).count();
                let score = product.salesrank as f32 / (cluster_size as f32).max(1.0);

                Some((product, score))
            })
            .collect();

        // Sort by score (ascending - lower score = better opportunity)
        scores.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        scores.into_iter().take(top_n).collect()
    }

    //Print dataset statistics
    pub fn print_stats(&self) {
        println!("\nDataset Statistics:");
        println!("- Products: {}", self.graph.node_count());
        println!("- Connections: {}", self.graph.edge_count());
        println!("- Avg connections per product: {:.2}", 
            self.graph.edge_count() as f32 / self.graph.node_count() as f32);
    }
}
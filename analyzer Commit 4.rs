use petgraph::algo::kosaraju_scc;
use petgraph::graph::Graph;
use std::collections::HashMap;

use crate::loader::ProductDataset;
use crate::Product;  // Import Product from main.rs

pub struct ProductAnalyzer<'a> {
    graph: &'a Graph<String, ()>,
    products: &'a HashMap<String, Product>,
}

impl<'a> ProductAnalyzer<'a> {
    pub fn new(dataset: &'a ProductDataset) -> Self {
        Self {
            graph: &dataset.graph,
            products: &dataset.products,
        }
    }

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

        products.sort_by(|(a_conn, _), (b_conn, _)| b_conn.cmp(a_conn));
        products.into_iter()
            .take(limit)
            .map(|(_, p)| p)
            .collect()
    }

    pub fn detect_trend_clusters(&self, min_size: usize) -> Vec<Vec<&'a Product>> {
        kosaraju_scc(self.graph)
            .into_iter()
            .filter(|c| c.len() >= min_size)
            .map(|cluster| {
                cluster.iter()
                    .filter_map(|n| self.products.get(&self.graph[*n]))
                    .collect()
            })
            .filter(|c: &Vec<_>| !c.is_empty())
            .collect()
    }

    pub fn find_low_competition_products(&self, top_n: usize) -> Vec<(&'a Product, f32)> {
        let mut scores: Vec<(&Product, f32)> = self.graph.node_indices()
            .filter_map(|node| {
                let asin = &self.graph[node];
                let product = self.products.get(asin)?;
                
                if product.salesrank <= 0 || product.salesrank > 100_000 {
                    return None;
                }

                let cluster_size = self.graph.neighbors(node).count();
                let connections = self.graph.edges(node).count();
                let score = connections as f32 / (cluster_size as f32).max(1.0);

                Some((product, score))
            })
            .collect();

        scores.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        scores.into_iter().take(top_n).collect()
    }

    pub fn print_stats(&self) {
        println!("\nDataset Statistics:");
        println!("- Products: {}", self.graph.node_count());
        println!("- Connections: {}", self.graph.edge_count());
        println!("- Avg connections per product: {:.2}", 
            self.graph.edge_count() as f32 / self.graph.node_count() as f32);
    }
}
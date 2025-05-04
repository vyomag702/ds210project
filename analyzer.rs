use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;

use crate::Product;
use crate::loader::ProductDataset;

pub struct ProductAnalyzer<'a> {
    pub graph: &'a Graph<String, ()>,
    pub products: &'a HashMap<String, Product>,
}

impl<'a> ProductAnalyzer<'a> {
    pub fn new(dataset: &'a ProductDataset) -> Self {
        Self {
            graph: &dataset.graph,
            products: &dataset.products,
        }
    }

    pub fn top_products_by_connections(&self, limit: usize) -> Vec<&'a Product> {
        let mut deg_and_prod: Vec<(usize, &Product)> = self
            .graph
            .node_indices()
            .filter_map(|idx: NodeIndex| {
                let asin = &self.graph[idx];
                self.products.get(asin).map(|prod| {
                    let degree = self.graph.neighbors(idx).count();
                    (degree, prod)
                })
            })
            .collect();

        deg_and_prod.sort_by_key(|(deg, _)| std::cmp::Reverse(*deg));
        deg_and_prod
            .into_iter()
            .take(limit)
            .map(|(_, prod)| prod)
            .collect()
    }
    pub fn detect_trend_clusters(&self) -> Vec<Vec<&'a Product>> {
        Vec::new()
    }
    pub fn find_low_competition_products(&self) -> Vec<(&'a Product, f32)> {
        Vec::new()
    }
}

use petgraph::graph::Graph;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::Product;

pub struct ProductDataset {
    pub graph: Graph<String, ()>,
    pub products: HashMap<String, Product>,
}
// Checking for the amazon data file and returns a 'ProdcutDataset' or an error
pub fn load_dataset(file_path: &str) -> Result<ProductDataset, Box<dyn Error>> {
    //adding the timer to check how long the import took
    let timer = Instant::now();
    
    
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("Dataset file not found: {}", file_path).into());
    }

    let mut graph = Graph::new();
    let mut products = HashMap::new();
    let mut node_indices = HashMap::new();
    let mut current_product: Option<Product> = None;

    println!("Loading dataset from: {}", file_path);
    let file_content = fs::read_to_string(file_path)?;

    for line in file_content.lines() {
        if line.starts_with("ASIN: ") {
            current_product = Some(Product {
                asin: line[6..].trim().to_string(),
                title: String::new(),
                group: String::new(),
                salesrank: -1,
            });
        } 
        else if let Some(ref mut product) = current_product {
            if line.starts_with("  title: ") {
                product.title = line[8..].trim().to_string();
            }
            else if line.starts_with("  group: ") {
                product.group = line[8..].trim().to_string();
            }
            else if line.starts_with("  salesrank: ") {
                product.salesrank = line[12..].trim().parse().unwrap_or(-1);
            }
            else if line.contains("similar:") && !product.title.is_empty() {
                // Ensure that the current product is finalized and link it to the ASINs that follow
                process_similar_products(&line, product, &mut graph, &mut node_indices, &mut products);
            }
        }
    }

    // Checking if the dataset is empty
    if products.is_empty() {
        return Err("Dataset appears to be empty - no valid products found".into());
    }

    println!("Dataset loaded in {:.2} seconds", timer.elapsed().as_secs_f32());
    println!("Products processed: {}", products.len());
    println!("Connections established: {}", graph.edge_count());

    Ok(ProductDataset { graph, products })
}

fn process_similar_products(
    line: &str,
    product: &Product,
    graph: &mut Graph<String, ()>,
    node_indices: &mut HashMap<String, petgraph::graph::NodeIndex>,
    products: &mut HashMap<String, Product>
) {
    products.entry(product.asin.clone())
        .or_insert_with(|| product.clone());

    let main_node = *node_indices
        .entry(product.asin.clone())
        .or_insert_with(|| graph.add_node(product.asin.clone()));

    let similar_products = line.split_whitespace().skip(2);
    
    for similar_asin in similar_products {
        let similar_node = *node_indices
            .entry(similar_asin.to_string())
            .or_insert_with(|| graph.add_node(similar_asin.to_string()));
        
        if !graph.contains_edge(main_node, similar_node) {
            graph.add_edge(main_node, similar_node, ());
        }
    }
}
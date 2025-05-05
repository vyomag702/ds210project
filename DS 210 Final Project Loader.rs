// Module for loading and parsing Amazon product dataset into graph structure

use petgraph::graph::Graph;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

// this struct represents an amazon product with key metadata
#[derive(Debug, Clone)]
pub struct Product {
    pub asin: String,   //Amazon standard identification number
    pub title: String,  // Product name
    pub group: String,  // Product category (Book, Music, etc.)
    pub salesrank: i32, // Sales ranking (lower = better selling)
}
// this struct is the container for the complete dataset including graph representation
pub struct ProductDataset {
    pub graph: Graph<String, ()>,          // Product relationship graph
    pub products: HashMap<String, Product>, // ASIN-to-Product mapping
}

/* Loads and parses Amazon product dataset from file
  Returns a result with ProductDataset or error message
  Logic
 1. Reads file line by line
 2. Parses product attributes (ASIN, title, etc.)
 3. Builds graph of product relationships
 */

pub fn load_dataset(file_path: &str) -> Result<ProductDataset, Box<dyn Error>> {
    let timer = Instant::now();
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("Dataset file not found: {}", file_path).into());
    }
    // Initialize data structures
    let mut graph = Graph::new();
    let mut products = HashMap::new();
    let mut node_indices = HashMap::new();
    let mut current_product: Option<Product> = None;

    println!("Loading dataset from: {}", file_path);
    let file_content = fs::read_to_string(file_path)?;
    // parsing loop
    for line in file_content.lines() {
        let line = line.trim();
        if line.is_empty() {
            current_product = None; // Empty line indicates record separator
            continue;
        }

        if line.starts_with("ASIN: ") {
            // Starting a new product record
            current_product = Some(Product {
                asin: line[6..].trim().to_string(),
                title: String::new(),
                group: String::new(),
                salesrank: -1,
            });
        } else if let Some(ref mut product) = current_product {
            // parsing product attributes
            if line.starts_with("title: ") {
                product.title = line[7..].trim().to_string();
            } else if line.starts_with("group: ") {
                product.group = line[7..].trim().to_string();
            } else if line.starts_with("salesrank: ") {
                product.salesrank = line[11..].trim().parse().unwrap_or(-1);
            } else if line.starts_with("similar: ") {
                // Processing product relationships
                process_similar_products(
                    line,
                    product,
                    &mut graph,
                    &mut node_indices,
                    &mut products,
                );
            }
        }
    }

    if products.is_empty() {
        return Err("Dataset appears to be empty - no valid products found".into());
    }

    println!("Dataset loaded in {:.2} seconds", timer.elapsed().as_secs_f32());
    println!("Products processed: {}", products.len());
    println!("Connections established: {}", graph.edge_count());

    Ok(ProductDataset { graph, products })
}
/* Processes similar products list and builds graph edges
   Arguments:
  'line' - Input line containing similar products
  'product' - Current product
  'graph' - Mutable reference to product graph
  'node_indices' - Mapping of ASINs to graph nodes
  'products - Products collection
  Logic:
 1. Adds current product to products map if not present
 2. Gets or creates graph node for current product
 3. Processes each similar product and creates edges*/

fn process_similar_products(
    line: &str,
    product: &Product,
    graph: &mut Graph<String, ()>,
    node_indices: &mut HashMap<String, petgraph::graph::NodeIndex>,
    products: &mut HashMap<String, Product>,
) {
    // ensures product is in collection
    products.entry(product.asin.clone())
        .or_insert_with(|| product.clone());
    // access or create a graph node
    let main_node = *node_indices
        .entry(product.asin.clone())
        .or_insert_with(|| graph.add_node(product.asin.clone()));

    let similar_products = line.split_whitespace().skip(2);
    
    for similar_asin in similar_products {
        let similar_node = *node_indices
            .entry(similar_asin.to_string())
            .or_insert_with(|| graph.add_node(similar_asin.to_string()));
        // Add edge if not already there
        if !graph.contains_edge(main_node, similar_node) {
            graph.add_edge(main_node, similar_node, ());
        }
    }
}
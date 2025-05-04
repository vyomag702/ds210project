mod loader;

#[derive(Debug, Clone)]
pub struct Product {
    pub asin: String,
    pub title: String,
    pub group: String,
    pub salesrank: i32,
}

fn main() {
    println!("Amazon Product Trend Analyzer");
    
    match loader::load_dataset("data/amazon-meta.txt") {
        Ok(dataset) => {
            println!("\nSuccessfully loaded dataset");
            println!("Products loaded: {}", dataset.products.len());
            println!("Co-purchase connections: {}", dataset.graph.edge_count());
        }
        Err(err) => {
            eprintln!("\nFailed to load dataset: {}", err);
            std::process::exit(1);
        }
    }
}
use surreal_repo::{model::Product, product_repository::ProductRepository};

#[tokio::main]
async fn main() {
    let repo = ProductRepository::new().await;

    let product = Product {
        id: None,
        name: "Shoes".to_string(),
        description: "A sport shoe".to_string(),
        price: 10.0,
        quantity: 9000,
    };

    let result = repo.create_product(product).await;
    match result {
        Ok(_) => println!("Product created successfully"),
        Err(e) => println!("Error creating product: {:?}", e),
    }
}

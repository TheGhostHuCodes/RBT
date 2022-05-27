async fn hello() {
    println!("Hello, world!");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    hello();
    Ok(())
}

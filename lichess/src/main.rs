use crate::world::LiWorld;
use dotenv::dotenv;
use licoricedev::client::Lichess;
use std::sync::Arc;

mod ligame;
mod world;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenv() {
        Ok(p) => println!("Succesfully read: {:?}", p),
        Err(e) => panic!("Could not read: {:?}", e),
    }
    let lichess = Arc::new(Lichess::new(String::from(std::env::var("LICHESS_TOKEN")?)));

    println!("Creating new world");
    let mut world = LiWorld::new(lichess);

    println!("Start listening");
    world.listen().await?;

    Ok(())
}

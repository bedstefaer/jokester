use icanhazdadjoke_sdk::{DadJokeSDK, PaginationOpts, models::Joke};
use std::sync::Arc;

async fn jokes()-> Result<icanhazdadjoke_sdk::models::PaginatedData<icanhazdadjoke_sdk::models::PaginatedJoke>, Box<dyn std::error::Error>> {
    let dad_joke_sdk = DadJokeSDK::new("NAME_YOUR_APP".into()); // you can basically just put in here what your app is doing
    // let _random_joke = dad_joke_sdk.get_random_joke().await; // gets a random joke
    let joke = dad_joke_sdk.query_jokes("pizza".into(), PaginationOpts::default()).await?; // you can specify more within the pagination options if you would like
    Ok(joke)
}

async fn get_random_joke(sdk:  Arc<DadJokeSDK>) -> Result<Joke,Box<dyn std::error::Error>> {
//    let stuff = sdk.get_random_joke().await;
}

#[tokio::main]
async fn main() {
    let joke = jokes().await;
    println!("{:?}",joke.unwrap().current_page);
    println!("Hello, world john!");

}

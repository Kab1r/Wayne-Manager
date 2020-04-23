use serenity::client::Client;

use std::env;

mod handler;
pub use handler::Handler;

fn main() {
    let mut client = Client::new(
        &env::var("DISCORD_TOKEN").expect("Missing Token in Enviroment"),
        Handler,
    )
    .expect("Error creating client");

    // Start with single shard
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why)
    }
}

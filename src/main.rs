use dotenv::dotenv;
use handler::Handler;
use serenity::{all::GatewayIntents, Client};
use std::{
    env,
    time::{Duration, Instant},
};
use tokio::time::sleep;

mod content_utils;
mod handler;

#[tokio::main]
async fn main() {
    let _ = dotenv();
    let token = env::var("TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    tokio::spawn(async move {
        let interval = Duration::from_secs(60 * 60 * 8);
        let mut next_time = Instant::now() + interval;
        loop {
            eprintln!("Updating yt-dlp");
            tokio::process::Command::new("yt-dlp")
                .arg("-U")
                .output()
                .await
                .expect("Failed to update yt-dlp");
            sleep(next_time - Instant::now()).await;
            next_time += interval;
        }
    });
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

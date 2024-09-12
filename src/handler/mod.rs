use std::process::Command;

use serenity::{all::{Context, EventHandler, Message}, async_trait};
use uuid::Uuid;

use crate::content_utils;

pub struct Handler;


#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        println!("{}: {}", msg.author.name, msg.content);

        if msg.author.id == ctx.cache.current_user().id {
            return;
        }

        let content = msg.content;

        if content.starts_with("!!") {
            println!("Skipping embed on {}", content);
            return;
        }

        let should_be_spoiled = content_utils::should_be_spoilered(&content);
        let is_valid = content_utils::is_valid(&content);
        
        println!("Message Created");
        println!("Author: {}", msg.author.name);
        println!("Content: {}", content);
        let check_url = regex::Regex::new(&content_utils::get_regex(&is_valid)).unwrap();
        let content = check_url.find(&content).unwrap().as_str();
        match is_valid {
            content_utils::Content::Twitter => {
                let cmd = Command::new("yt-dlp")
                    .arg("-g")
                    .arg("-f")
                    .arg("best[ext=mp4]")
                    .arg(content);
                let output = cmd.output().expect("Failed to execute command");
                msg.reply(ctx, format!("[Twitter Video]({})", String::from_utf8(output.stdout).unwrap())).await;
            },
            _ => {
               (output, outPath) = content_utils::download(content, should_be_spoiled);
            }
        }
   }
}
pub fn download(content: &str, should_be_spoiled: bool) -> (String, String) {
    let file_name = Uuid::new_v4().to_string();
}

use std::process::Command;

use serenity::{
    all::{Context, CreateAllowedMentions, CreateAttachment, CreateMessage, EventHandler, Message},
    async_trait,
};

use crate::content_utils;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.current_user().id {
            return;
        }

        let content = &msg.content;

        if content.starts_with("!!") {
            println!("Skipping embed on {}", content);
            return;
        }

        let should_be_spoiled = content_utils::should_be_spoilered(content);
        let is_valid = content_utils::is_valid(content);

        let check_url = regex::Regex::new(&content_utils::get_regex(&is_valid)).unwrap();
        let content = check_url.find(content).unwrap().as_str();
        if is_valid == content_utils::Content::None {
            return;
        }
        println!("Message Created");
        println!("Author: {}", msg.author.name);
        println!("Content: {}", content);

        match is_valid {
            content_utils::Content::Twitter => {
                let mut cmd = Command::new("yt-dlp");
                let command = cmd.arg("-g").arg("-f").arg("best[ext=mp4]").arg(content);
                let output = command.output().expect("Failed to execute command");
                if output.stdout.is_empty() {
                    return;
                }
                if let Err(why) = &msg
                    .reply(
                        ctx,
                        format!(
                            "[Twitter Video]({})",
                            String::from_utf8(output.stdout).unwrap()
                        ),
                    )
                    .await
                {
                    println!("Error sending message: {:?}", why);
                };
            }
            _ => {
                // the file is stored @ out_path
                let (output, out_path) = content_utils::download(content, should_be_spoiled).await;

                println!("Output: {}", output);
                println!("OutPath: {}", out_path);
                let files = CreateAttachment::path(&out_path).await.unwrap();
                let allowed_mentions = CreateAllowedMentions::default().replied_user(false);
                let message = CreateMessage::new()
                    .reference_message(&msg)
                    .files(vec![files])
                    .allowed_mentions(allowed_mentions);
                let _ = msg.channel_id.send_message(&ctx.http, message).await;

                if let Err(why) = std::fs::remove_file(&out_path) {
                    println!("Error deleting file: {:?}", why);
                }
            }
        }
    }
}

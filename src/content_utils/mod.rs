use std::{collections::HashMap, hash::Hasher, process::Command};

use uuid::Uuid;
pub fn should_be_spoilered(content: &str) -> bool {
    let pattern = r"^([|]{2}).*([|]{2})$";
    let re = regex::Regex::new(pattern).unwrap();
    re.is_match(content)
}
#[derive(PartialEq, Hash, Eq, Debug)]
pub enum Content {
    Twitter,
    Tiktok,
    Reddit,
    Instagram,
    Facebook,
    None,
}

impl Clone for Content {
    fn clone(&self) -> Content {
        match self {
            Content::Twitter => Content::Twitter,
            Content::Tiktok => Content::Tiktok,
            Content::Reddit => Content::Reddit,
            Content::Instagram => Content::Instagram,
            Content::Facebook => Content::Facebook,
            Content::None => Content::None,
        }
    }
}
pub fn get_regex(content: &Content) -> String {
    let regex_map: HashMap<Content, String> = HashMap::from([
        (
            Content::Twitter,
            r"https:\/\/(?:www\.)?(twitter|x)\.com\/.+\/status(?:es)?\/(\d+)(?:.+ )?".to_string(),
        ),
        (
            Content::Tiktok,
            r"https?://(?:www\.|vm\.|vt\.)?tiktok\.com/.+(?: )?".to_string(),
        ),
        (
            Content::Reddit,
            r"https?://(?:(?:old\.|www\.)?reddit\.com|v\.redd\.it)/.+(?: )?".to_string(),
        ),
        (
            Content::Instagram,
            r"https?:\/\/(?:www\.)?instagram\.com\/[a-zA-Z0-9_]+\/?(?:\?igshid=[a-zA-Z0-9_]+)?"
                .to_string(),
        ),
        (
            Content::Facebook,
            r"https?:\/\/(?:www\.)?facebook\.com\/(reel)\/[a-zA-Z0-9_]+\/?".to_string(),
        ),
    ]);
    let winner = regex_map.iter().find(|(key, _)| key == &content);
    match winner {
        Some((_, value)) => value.to_string(),
        None => panic!("No valid content found"),
    }
}
pub fn is_valid(content: &str) -> Content {
    let regex_map: HashMap<Content, String> = HashMap::from([
        (
            Content::Twitter,
            r"https:\/\/(?:www\.)?(twitter|x)\.com\/.+\/status(?:es)?\/(\d+)(?:.+ )?".to_string(),
        ),
        (
            Content::Tiktok,
            r"https?://(?:www\.|vm\.|vt\.)?tiktok\.com/.+(?: )?".to_string(),
        ),
        (
            Content::Reddit,
            r"https?://(?:(?:old\.|www\.)?reddit\.com|v\.redd\.it)/.+(?: )?".to_string(),
        ),
        (
            Content::Instagram,
            r"https?:\/\/(?:www\.)?instagram\.com\/[a-zA-Z0-9_]+\/?(?:\?igshid=[a-zA-Z0-9_]+)?"
                .to_string(),
        ),
        (
            Content::Facebook,
            r"https?:\/\/(?:www\.)?facebook\.com\/(reel)\/[a-zA-Z0-9_]+\/?".to_string(),
        ),
    ]);
    let winner = regex_map.iter().find(|(_, value)| {
        let re = regex::Regex::new(value).unwrap();
        re.is_match(content)
    });
    match winner {
        Some((key, _)) => key.clone(),
        None => Content::None,
    }
}
pub async fn download(content: &str, should_be_spoiled: bool) -> (String, String) {
    let mut file_name = Uuid::new_v4().to_string() + ".mp4";
    if should_be_spoiled {
        file_name = "SPOILER_".to_string() + &file_name;
    }
    let mut binding = Command::new("yt-dlp");
    let command = binding
        .arg("-f")
        .arg("bestvideo[filesize<30MB]+bestaudio[filesize<10mb]/best/bestvideo+bestaudio")
        .arg("-S")
        .arg("vcodec:h264")
        .arg("--merge-output-format")
        .arg("mp4")
        .arg("--ignore-config")
        .arg("--verbose")
        .arg("--no-playlist")
        .arg("--no-warnings")
        .arg("-o")
        .arg(&file_name)
        .arg(content);
    let output = command.output().expect("Failed to execute command");
    (String::from_utf8(output.stdout).unwrap(), file_name)
}

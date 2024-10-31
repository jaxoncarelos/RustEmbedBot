use std::{io::Error, path::PathBuf};

use regex::RegexSet;
use tokio::process::Command;
use uuid::Uuid;

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
pub enum Content {
    Twitter,
    Some,
    None,
}
pub fn should_be_spoilered(content: &str) -> bool {
    let pattern = r"^([|]{2}).*([|]{2})$";
    let re = regex::Regex::new(pattern).unwrap();
    re.is_match(content)
}
static TWITTER_REGEX: &str =
    r"https:\/\/(?:www\.)?(twitter|x)\.com\/.+\/status(?:es)?\/(\d+)(?:.+ )?";
static REGEX_MAP: [&str; 4] = [
    r"https?://(?:www\.)?(?:vm\.|vt\.)tiktok\.com/.+",
    r"https?://(?:(?:old\.|www\.)?reddit\.com|v\.redd\.it)/.+(?: )?",
    r"https?:\/\/(?:www\.)?instagram\.com\/[a-zA-Z0-9_]+\/?(?:\?igshid=[a-zA-Z0-9_]+)?",
    r"https?:\/\/(?:www\.)?facebook\.com\/(reel)\/[a-zA-Z0-9_]+\/?",
];

pub fn get_regex(content: String) -> String {
    if regex::Regex::new(TWITTER_REGEX).unwrap().is_match(&content) {
        return TWITTER_REGEX.to_string();
    }
    let set = RegexSet::new(REGEX_MAP).unwrap();
    let matches = set.matches(&content);

    if matches.iter().count() == 0 {
        return "".to_string();
    }
    return REGEX_MAP[matches.iter().next().unwrap()].to_string();
}
pub fn is_valid(content: &str) -> Content {
    if regex::Regex::new(TWITTER_REGEX).unwrap().is_match(content) {
        return Content::Twitter;
    }

    let set = RegexSet::new(REGEX_MAP).unwrap();
    let content = content.trim();
    let matches = set.matches(content);
    if matches.iter().count() > 0 {
        return Content::Some;
    }
    Content::None
}
// Downloads the content from the URL using predefined yt-dlp command.
// Returns the output and the file name.
// The file name is a UUID v4 string with the .mp4 extension.
// If the content should be spoilered, the file name is prefixed with "SPOILER_".
// The file is stored in the current working directory.
pub async fn download(content: &str, should_be_spoiled: bool) -> Result<(String, PathBuf), Error> {
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
    let output = command.output().await?;
    if output.status.success() {
        Ok((
            String::from_utf8(output.stdout).unwrap(),
            PathBuf::from(file_name),
        ))
    } else {
        Err(Error::new(
            std::io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ))
    }
}

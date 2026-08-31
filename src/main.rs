mod config;
mod data;
mod json_util;
mod mail_util;
mod rss_util;
use std::path::Path;

fn main() {
    // Read config-file
    let config_path = Path::new("config.json");
    let config: Result<config::Config, Box<dyn std::error::Error>> =
        json_util::get_from_json(config_path);
    let config: config::Config = match config {
        Ok(config) => config,
        Err(error) => panic!("{} config error", error),
    };

    // Read data-file
    let data_path = Path::new("data.json");
    let data: Result<data::Data, Box<dyn std::error::Error>> = json_util::get_from_json(data_path);
    let mut data: data::Data = match data {
        Ok(data) => data,
        Err(error) => panic!("{}", error),
    };

    // Get latest post
    let latest_post: Result<rss::Item, Box<dyn std::error::Error>> = rss_util::get_latest_post();
    let latest_post: rss::Item = match latest_post {
        Ok(post) => post,
        Err(error) => panic!("{}", error),
    };

    let content: String = match latest_post.content {
        Some(content) => content,
        None => "".to_string(),
    };

    let current_blog_title: String = match latest_post.title {
        Some(title) => title,
        None => "".to_string(),
    };

    let search_string: String = format!("{} {}", content, current_blog_title);

    let regex_matched: bool = check_for_regex(&search_string, &config.regexes);

    if regex_matched == true && current_blog_title != data.last_blog_title {
        let _ = mail_util::send_mail(&config, "Regex matched", &content);
    }

    if current_blog_title != data.last_blog_title {
        data.last_blog_title = current_blog_title;
        let _ = json_util::save_to_json_file(data, data_path);
    }
}

fn check_for_regex(text: &str, regexes: &Vec<String>) -> bool {
    let mut pattern_matched: bool = false;
    let text = text.to_lowercase();

    for regex in regexes.iter() {
        if text.contains(&regex.to_lowercase()) {
            pattern_matched = true;
        }
    }
    return pattern_matched;
}

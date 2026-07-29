use std::path::Path;
mod config;
mod data;
mod json_util;
mod rss_util;

fn main() {
    // Read config-file
    let config_path = Path::new("./src/config/config.json");
    let config: Result<config::Config, Box<dyn std::error::Error>> =
        json_util::get_from_json(config_path);
    let config: config::Config = match config {
        Ok(config) => config,
        Err(error) => panic!("{}", error),
    };

    // Read data-file
    let data_path = Path::new("./src/config/data.json");
    let data: Result<data::Data, Box<dyn std::error::Error>> = json_util::get_from_json(data_path);
    let data: data::Data = match data {
        Ok(data) => data,
        Err(error) => panic!("{}", error),
    };

    // Get latest post
    let latest_post: Result<rss::Item, Box<dyn std::error::Error>> = rss_util::get_latest_post();
    let latest_post: rss::Item = match latest_post {
        Ok(post) => post,
        Err(error) => panic!("{}", error),
    };

    if latest_post.title().unwrap() != data.last_blog_title {
        let _ = update_data(data, latest_post.title().unwrap(), data_path);
    }

    let regex_matched: bool =
        check_for_regex(&latest_post.content.clone().unwrap(), config.regexes);

    if regex_matched == true {
        // ToDo
    }
}

fn update_data(
    mut data: data::Data,
    value: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    data.last_blog_title = value.to_string();
    return json_util::save_to_json_file(data, path);
}

fn check_for_regex(text: &str, regexes: Vec<String>) -> bool {
    let mut pattern_matched: bool = false;
    let text = text.to_lowercase();

    for regex in regexes.iter() {
        if text.contains(&regex.to_lowercase()) {
            pattern_matched = true;
        }
    }
    return pattern_matched;
}

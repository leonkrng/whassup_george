use reqwest::blocking::Response;
use rss::Channel;
use std::{fs::File, io::{BufReader, Cursor}};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Config {
    sender_email: String,
    receiver_email: String
}

#[derive(Serialize, Deserialize)]
struct Data {
    last_blog_title: String,
}

fn main() {
    let latest_post: Result<rss::Item, Box<dyn std::error::Error>> = get_latest_post();

    let latest_post: rss::Item = match latest_post {
        Ok(post) => post,
        Err(error) => panic!("{}", error)
    };

    println!("Feed Title: {:?}", latest_post.title);

    let config_path = Path::new("./src/config/config.json");
    let config: Result<Config, Box<dyn std::error::Error>>= get_from_json(config_path);
    let config: Config = match config  {
        Ok(config) => config,
        Err(error) => panic!("{}", error),
    };
    println!("{:?}, {:?}", config.sender_email, config.receiver_email);

    let data_path = Path::new("./src/config/data.json");
    let data: Result<Data, Box<dyn std::error::Error>> = get_from_json(data_path);
    let data: Data = match data {
        Ok(data) => data,
        Err(error) => panic!("{}", error),
    };
    println!("{:?}", data.last_blog_title);
}

fn get_latest_post() -> Result<rss::Item, Box<dyn std::error::Error>> {

    let body: Response = reqwest::blocking::get("https://georgerrmartin.com/notablog/feed")?;
    let bytes = body.bytes()?;
    let channel = Channel::read_from(Cursor::new(bytes))?;

    return Ok(channel.items()[0].clone());
}

fn get_from_json<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);


    let u = serde_json::from_reader(reader)?;
    return Ok(u);
}

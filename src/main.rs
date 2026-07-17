use reqwest::blocking::Response;
use rss::Channel;
use std::io::Cursor;

fn main() {
    let latest_post: Result<rss::Item, Box<dyn std::error::Error>> = get_latest_post();

    let latest_post: rss::Item = match latest_post {
        Ok(post) => post,
        Err(error) => panic!("{}", error)
    };

    println!("Feed Title: {:?}", latest_post.title);
}

fn get_latest_post() -> Result<rss::Item, Box<dyn std::error::Error>> {

    let body: Response = reqwest::blocking::get("https://georgerrmartin.com/notablog/feed")?;
    let bytes = body.bytes()?;
    let channel = Channel::read_from(Cursor::new(bytes))?;

    return Ok(channel.items()[0].clone());
}

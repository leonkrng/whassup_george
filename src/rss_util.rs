use reqwest::blocking::Response;
use rss::Channel;
use std::io::Cursor;

pub fn get_latest_post() -> Result<rss::Item, Box<dyn std::error::Error>> {

    let body: Response = reqwest::blocking::get("https://georgerrmartin.com/notablog/feed")?;
    let bytes = body.bytes()?;
    let channel = Channel::read_from(Cursor::new(bytes))?;

    return Ok(channel.items()[0].clone());
}

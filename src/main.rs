use rss::Channel;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get(
        "https://georgerrmartin.com/notablog/feed",
    )?
    .bytes()?;

    let channel = Channel::read_from(Cursor::new(body))?;

    println!("Feed Title: {}", channel.title());

    Ok(())
}

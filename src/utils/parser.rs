use rss::Channel;
use std::fs::File;
use std::io::{BufReader, Error};

pub trait Parser<T> {
    fn parse(filepath: &str) -> Result<Vec<T>, Error>;
}

pub struct FeedParser;

#[test]
fn parse_valid_xml() {
    let file = File::open("tests/valid_feed.xml").unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();
    channel.write_to(::std::io::sink()).unwrap();
    let string = channel.to_string();
    println!("{}", string);
}

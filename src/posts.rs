use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TITLE_REGEX: Regex = Regex::new(r"^\[(?P<country>..)(?:-(?P<state>..))?\]\s*\[H\](?P<have>.*)\[W\](?P<want>.*)$").unwrap();
    static ref COMMA_SEPARATED_REGEX: Regex = Regex::new(r"\s*,\s*").unwrap();
}

#[derive(Debug)]
pub struct PostTitle<'t> {
    country: &'t str,
    state: Option<&'t str>,
    have: Vec<&'t str>,
    want: Vec<&'t str>,
}

pub fn parse_post_title(post_title: &str) -> Result<PostTitle, Box<dyn std::error::Error>> {
    let captures = TITLE_REGEX.captures(post_title).ok_or("Failed to parse post title")?;

    Ok(PostTitle {
        country: captures.name("country").unwrap().as_str(),
        state: captures.name("state").map(|m| m.as_str()),
        have: split_items(captures.name("have").unwrap().as_str().trim()),
        want: split_items(captures.name("want").unwrap().as_str().trim()),
    })
}

fn split_items(items: &str) -> Vec<&str> {
    COMMA_SEPARATED_REGEX.split(items).collect()
}
use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TITLE_REGEX: Regex = Regex::new(r"^\[(?P<country>..)(?:-(?P<state>..))?\]\s*\[H\](?P<have>.*)\[W\](?P<want>.*)$").unwrap();
    static ref COMMA_SEPARATED_REGEX: Regex = Regex::new(r"\s*,\s*").unwrap();
}

pub struct PostTitle<'t> {
    country: &'t str,
    state: Option<&'t str>,
    have: Vec<&'t str>,
    want: Vec<&'t str>,
}

impl fmt::Display for PostTitle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.state.map(|state| format!("-{}", state)).unwrap_or_default();
        write!(f, "[{}{}] [H] {} [W] {}", self.country, state, self.have.join(", "), self.want.join(", "))
    }
}

impl PostTitle<'_> {
    pub fn new(post_title: &str) -> Option<PostTitle> {
        let title_captures = TITLE_REGEX.captures(post_title);

        title_captures.map(|captures| PostTitle {
            country: captures.name("country").unwrap().as_str(),
            state: captures.name("state").map(|m| m.as_str()),
            have: split_items(captures.name("have").unwrap().as_str().trim()),
            want: split_items(captures.name("want").unwrap().as_str().trim()),
        })
    }
}

fn split_items(items: &str) -> Vec<&str> {
    COMMA_SEPARATED_REGEX.split(items).collect()
}
use regex::Regex;

#[derive(Debug)]
pub struct Emoji {
    pub code: &'static str,
    pub description: &'static str,
    pub emoji: &'static str,
    pub entity: &'static str,
    pub name: &'static str,
}

impl Emoji {
    pub fn contains(&self, pattern: &Regex) -> bool {
        pattern.is_match(self.code)
            || pattern.is_match(self.description)
            || pattern.is_match(self.emoji)
            || pattern.is_match(self.entity)
            || pattern.is_match(self.name)
    }
}

include!(concat!(env!("OUT_DIR"), "/emojis.rs"));

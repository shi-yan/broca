use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Lang {
    English(String),
    Chinese(String),
    Spanish(String),
    Japanese(String),
    Korean(String),
    German(String),
    French(String),
    Portuguese(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub meaning: Vec<Lang>,
    pub examples: Vec<Vec<Lang>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meaning {
    pub pos: String,
    pub meanings: Vec<Item>
}

pub type Meanings = Vec<Meaning>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub query: String,
    pub meanings: Meanings
}

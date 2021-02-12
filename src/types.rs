pub const ARTICLE_COUNT: i32 = 200;

pub const MECHANICAL_SYMPATHY_DIAL: usize = 100; // how may concurrent  requests we will make

//wrap a handy response type for prettier code
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Default)]
pub struct Record {
    pub id: String,
    pub uri: String,
    pub title: String,
    pub stems: Vec<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct WikiResponse {
    pub query: Pages,
}

#[derive(Deserialize, Debug, Default)]
pub struct Pages {
    pub pages: std::collections::HashMap<String, Page>,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    pub pageid: i32,
    pub title: String,
    pub fullurl: String,
    pub extract: String,
}

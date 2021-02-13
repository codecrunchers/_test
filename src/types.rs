pub const ARTICLE_COUNT: i32 = 2;

pub const MECHANICAL_SYMPATHY_DIAL: usize = 100; // how may concurrent  requests we will make

//wrap a handy response type for prettier code
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Default)]
pub struct Record {
    pub id: String,
    pub uri: String,
    pub title: String,
    //This is a Lookup table for word and occurences, the Hashmap is order n, btree would be faster
    //if we were updating data only
    pub stems: std::collections::HashMap<String, u32>,
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

use serde_json::Value;

pub const ARTICLE_COUNT: i32 = 2;

#[derive(Deserialize, Debug)]
pub struct PartialResponse {
    //pub query: std::collections::HashMap<String, std::collections::HashMap<String, Page>>,
    pub query: Pages, // std::collections::HashMap<String, std::collections::HashMap<String, Page>>,
}

#[derive(Deserialize, Debug)]
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

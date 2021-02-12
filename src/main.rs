#[macro_use]
extern crate serde_derive;

use serde_json::Result;

mod types;
use futures::stream::StreamExt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use types::{PartialResponse, ARTICLE_COUNT};

fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(std::result::Result::ok).collect())
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    //stemmer("".into()).await;

    let paths: Vec<String> = read_lines("urls.txt")?;
    let articles = (1..ARTICLE_COUNT).collect::<Vec<i32>>();
    let fetches = futures::stream::iter(articles.into_iter().map(|_| async move {
        match reqwest::get("https://en.wikipedia.org/w/api.php?action=query&generator=random&grnnamespace=0&grnlimit=1&prop=info%7Cextracts&inprop=url&explaintext&format=json").await {
            Ok(resp) => match resp.text().await {
                Ok(text) => {
                    //println!("RESPONSE: {} bytes from {} yielding: {}", text.len(), "path", text);
                    let response : PartialResponse = serde_json::from_str(&text).unwrap();
                    println!("Extract : {:?}", response.query.pages);
                }
                Err(_) => println!("ERROR reading {}", "path"),
            },
            Err(_) => println!("ERROR downloading {}", "path"),
        }
    }))
    .buffer_unordered(100)
    .collect::<Vec<()>>();
    fetches.await;

    Ok(())
}

async fn stemmer(text: String) -> String {
    use porter_stemmer::stem;
    use unicode_segmentation::UnicodeSegmentation;

    let original = "Almost cat cat dog    phone mobilephone cellphone forty  dogs  years  later,  these  fair information  practices  have  become  the standard  for  privacy  protection  around  the  world.  And  yet,  over  that same time  period,  we  have  seen  an  exponential  growth  in  the  use  of surveillance technologies,  and  our  daily  interactions  are  now  routinely  captured, recorded, and manipulated by small and large institutions alike.";

    let tokenised_sentence = original.clone().unicode_words();

    println!("Original:\n{}", original);
    let results = tokenised_sentence
        .map(stem)
        .fold(String::new(), |last, next| format!("{}{} ", last, next));
    println!("Stemmed:\n{}", results);
    results
}

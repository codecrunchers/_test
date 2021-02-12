#[macro_use]
extern crate serde_derive;
mod types;

use futures::stream::StreamExt;
use types::{WikiResponse, ARTICLE_COUNT, Record, Result, MECHANICAL_SYMPATHY_DIAL};
use serde_json::from_str as marshaller;
use std::{io, io::prelude::*};
use porter_stemmer::stem;
use unicode_segmentation::UnicodeSegmentation;


#[tokio::main]
async fn main() -> Result<()>{
    build_database().await.and_then(|db| enable_search_mode(db))
}

/// Combine parallel http requests with thread interleaved async io 
/// to get ARTICLE_COUNT wikipedia records with full extracts in plaintext
/// WikiP have better processors than us, and String parsing is xxpensive, 
/// this way we do not have to filter out <markdown tags>///
/// # Returns a List of index_pages, meaning a page id, uri and stems
async fn build_database() -> Result<Vec<Record>> {
    let article_count = (1..ARTICLE_COUNT + 1).collect::<Vec<i32>>();
    println!("Fetching {} Random Wikipedia articles", ARTICLE_COUNT);

    let db =  futures::stream::iter(article_count.into_iter().map(|_| async move {
        match reqwest::get("https://en.wikipedia.org/w/api.php?action=query&generator=random&grnnamespace=0&grnlimit=1&prop=info%7Cextracts&inprop=url&explaintext&format=json").await {
            Ok(resp) => match resp.text().await {
                Ok(json) => {
                    let response :WikiResponse = marshaller(&json).unwrap_or(Default::default());
                    response.query.pages.iter().map(|page| Record {
                        id: page.0.to_string(),
                        uri: page.1.fullurl.clone().to_string(),
                        stems: stemmer(format!("{} {}",page.1.title.clone().to_string(),page.1.extract.to_string()), page.1.fullurl.to_string()),
                        title: page.1.title.to_string(),
                    }).collect::<Vec<Record>>()
                }
                Err(_) => Default::default()
            },
            Err(_) => Default::default()
        }
    }))
    .buffer_unordered(MECHANICAL_SYMPATHY_DIAL) 
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .flat_map(|indexed_page| indexed_page)       
        .filter(|indexed_page| indexed_page.id != "") //failed calls stipped
        .collect::<Vec<Record>>();

    Ok(db)

}

///Open up stdin , wait for a keywords and kick off search
///
/// # Arguments
///
/// * `db` - The 'database'
fn enable_search_mode(mut db: Vec<Record>) ->  Result<()>{

    println!("Welcome to WikiSearch: enter a keyword");

    for line in io::stdin().lock().lines() {
        //println!("length = {}", line?.len());
        let search_results : Vec<()> = db.iter_mut()
            .filter(|r| r.stems.contains(
                    &stem(
                        &line.as_ref()
                        .unwrap_or(&"".to_string())))
                )
            .map(|r|  println!("Title: {:?} ({:?}) ", r.title, r.uri,) )
            .collect();

        println!("{:?} Results",search_results.len());

    }

    /*let search_results : Vec<()> = db.into_iter()
      .filter(|r| r.stems.contains(line))
      .map(|r|  println!("{:?}", r.uri) )
      .collect();
      */
    //    }
Ok(())
    }


/// Generate a porter_stemmer based list of word stems from the article
///
/// # Arguments
///
/// * `text` - the plaintext article from wikipedia
fn stemmer(text: String, uri: String) -> Vec<String> {

    println!("Stemming..{}    " , uri);
    let original = text.as_str();
    let tokenised_sentence = original.clone().unicode_words();
    tokenised_sentence.map(stem).collect::<Vec<String>>()
}

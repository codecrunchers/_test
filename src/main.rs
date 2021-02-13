#[macro_use]
extern crate serde_derive;
mod types;
mod rankers;

use futures::stream::StreamExt;
use types::{WikiResponse, ARTICLE_COUNT, Record, Result, MECHANICAL_SYMPATHY_DIAL};
use serde_json::from_str as marshaller;
use std::{io, io::prelude::*};
use porter_stemmer::stem;
use unicode_segmentation::UnicodeSegmentation;
use rankers::{NaiveRanker, Ranker};


#[tokio::main]
async fn main() -> Result<()>{
    build_database().await.and_then(|db| enable_search_mode(db))
}

/// Combine parallel http requests with thread interleaved async io 
/// to get ARTICLE_COUNT wikipedia records with full extracts in plaintext
/// WikiP have better processors than us, and String parsing is expensive, 
/// this way we do not have to filter out <markdown tags>///
/// # Returns a List of indexed_pages, i.e { a page id, uri, stems (from stemming Alg)}
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
        .filter(|indexed_page| indexed_page.id != "") //failed calls stripped
        .collect::<Vec<Record>>();


    Ok(db)

}

///Open up stdin , wait for a keywords and kick off search
///
/// # Arguments
///
/// * `db` - The 'database'
/// Returns ()  - nothing
fn enable_search_mode(mut db: Vec<Record>) ->  Result<()> {

   println!("\r\n\r\nWelcome to WikiSearch: enter a keyword");
   let results = NaiveRanker::rank(&db, "alan".to_string());
   let mut results = results.unwrap();
   results.sort();
   println!("{:?}", results);

    for line in io::stdin().lock().lines() {
        let search_results : Vec<()> = db.iter_mut()
            .filter(|r| r.stems.contains(
                    &stem(
                        &line.as_ref()
                        .unwrap_or(&"".to_string())))
            )
            .map(|r|  println!("Title: {:?} ({:?}) ", r.title, r.uri,) )
            .collect();


        println!("{:?} Results", search_results.len());

    }

    Ok(())
}


/// Generate a porter_stemmer based list of 'stems' from the article text
///
/// # Arguments
///
/// * `text` - the plaintext article from wikipedia
/// * `uri` - for feedback only
fn stemmer(text: String, uri: String) -> Vec<String> {
    print!("Stemming..{} ..........   " , uri);
    let tokenised_sentence = text.as_str().clone().unicode_words();
    tokenised_sentence.map(stem).collect::<Vec<String>>()
}


#[cfg(test)]
mod tests{
    use crate::stemmer;

    #[test]
    fn test_stemmer(){
        let sentence =" he ran she runs they run he is a runner".to_string();
        assert_eq!(10, stemmer(sentence.clone(), "".into()).len(), "{:?}", stemmer(sentence, "".into()));

        let sentence = "".to_string();
        assert_eq!(0, stemmer(sentence.clone(), "".into()).len(), "{:?}", stemmer(sentence, "".into()))
    }
}

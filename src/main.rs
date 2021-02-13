///! Wikipedia Search App -  Entry point for application
///1. Fetch number of articles, 'stem' the plaintext body and sum the word occurence per link/hit
///2. Enable Search; first find matching keywords, then applies an implementation of a chosen Ranker

#[macro_use]
extern crate serde_derive;
mod types;
mod rankers;
mod stemmers;
use std::collections::HashMap;
use futures::stream::StreamExt;
use types::{WikiResponse, ARTICLE_COUNT, Record, Result, MECHANICAL_SYMPATHY_DIAL};
use serde_json::from_str as marshaller;
use std::{io, io::prelude::*};
use porter_stemmer::stem;
use rankers::{Ranker, WordCountRanker};
use log::{info,debug};
use std::time::Instant;
use stemmers::{SimplePorterStemmer, IStemmer};


//Availabe text Stemmers
enum Stemmer { 
    porter,
    otherNotImplmentedYet,
}


//Availabe Ranking Algs
enum RankerAlg {
    wordCount,
    otherNotImplmentedYet
}


#[tokio::main]
async fn main() -> Result<()>{
    env_logger::init();
    let start = Instant::now();

    build_database(Stemmer::porter).await.and_then(|db| {
        println!("Fetch and Index time: {:?}", start.elapsed());
        enable_search_mode(db, RankerAlg::wordCount)
    })
}

/// Combine parallell HTTP requests with thread interleaved async i/o 
/// to get ARTICLE_COUNT wikipedia records;  with full extracts in plaintext
/// As WikiP have better processors than us and string parsing is expensive, 
/// we do not have to filter out <markdown tags>///
///
/// # Returns 
/// * a List of indexed_pages, i.e { a page id, uri, stems (from stemming Alg)}
async fn build_database(stemmer: Stemmer) -> Result<Vec<Record>> {
    let article_count = 1..ARTICLE_COUNT + 1;//.collect::<Vec<i32>>();
    println!("Fetching {} Random Wikipedia articles", ARTICLE_COUNT);

    let stemmer = match stemmer {
        _porter => SimplePorterStemmer{},
    };

    let db =  futures::stream::iter(article_count.into_iter().map(|_| async move {
        match reqwest::get("https://en.wikipedia.org/w/api.php?action=query&generator=random&grnnamespace=0&grnlimit=1&prop=info%7Cextracts&inprop=url&explaintext&format=json").await {
            Ok(resp) => match resp.text().await {
                Ok(json) => { //now build the Response Object 
                    let response :WikiResponse = marshaller(&json).unwrap_or_default(); 
                    response.query.pages.iter().map(|page| Record {
                        id: page.0.to_string(),
                        uri: page.1.fullurl.to_string(),
                        stems:  tally_words(stemmer.istem(format!("{} {}",page.1.title ,page.1.extract.to_string()))),
                        title: page.1.title.to_string(),
                    }).collect::<Vec<Record>>()
                }
                Err(_) => Default::default()
            },
            Err(_) => Default::default()
        }
    })).buffer_unordered(MECHANICAL_SYMPATHY_DIAL)
    .collect::<Vec<_>>()
        .await 
        .into_iter()
        .flatten() //flatten the nested structure
        .filter(|indexed_page| !indexed_page.id.is_empty()) //failed calls stripped
        .collect::<Vec<Record>>();


    Ok(db)

}

///Open up stdin , wait for a keywords and kick off search
///
/// # Arguments
///
/// * `db` - The 'database'
/// # Returns 
/// * () / nothing
fn enable_search_mode(mut db: Vec<Record>, ranking_alg: RankerAlg) ->  Result<()> {

    println!("{}", "\r\n\r\nWelcome to WikiSearch: enter a keyword, ^C to exit");

    let alg = match ranking_alg {
        _wordCount => WordCountRanker,
    };

    for keyword in io::stdin().lock().lines()  {
        let start = Instant::now();
        println!();
        match &keyword.as_ref() {
            Ok(keyword) => {
                let search_results :Vec<(u32,String)> = alg.clone().rank(
                    db.iter_mut()
                    .filter(|record| 
                        record.stems.contains_key(&stem(&keyword.as_str().to_lowercase()))//find Records with matching stems in DB
                    )  
                    .collect::<Vec<_>>(),
                    stem(&keyword.as_str().to_lowercase()))
                    .unwrap();

                debug!("{:?}", search_results);

                for (rank, search_result) in search_results.iter().enumerate() {
                    println!("{} \t (hits {})\t{}", rank+1, search_result.0, search_result.1);
                }

                println!("{:?} Results for {:?} in {:?} \r\n",
                    search_results.len(),            
                    &keyword,
                    start.elapsed(),
                );
            },
            Err(e) => info!("{:?}", e)
        }
    }

    Ok(())
}




/** 
 * With rt-tokio, these are being run by tasks after i/o commpletes on 
 * the allocated thread
 **/
fn tally_words(untallied: Vec<String>) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    for word in untallied.iter()
        .filter(|word| !word.is_empty()) {
            *counts.entry(word.to_owned()).or_insert(0u32) += 1u32;
        }

    counts
}


#[cfg(test)]
mod tests{
    use crate::{tally_words, SimplePorterStemmer, IStemmer};

    #[test]
    fn test_tally_ho(){
        let sentence =" he ran she runs they run he is a runner".to_string();
        let response: std::collections::HashMap<String,u32> = [("he".into(),2),("a".into(),1),("she".into(),1),("run".into(),2),("ran".into(),1),("runner".into(),1),("thei".into(),1),("is".into(),1)].iter().cloned().collect();
        let stemmer  = SimplePorterStemmer{};
        assert_eq!(response,tally_words(stemmer.istem(sentence.clone())))
    }

}

///! These tare the Ranker implementations, allowing for a algorithm to use R Where T:Ranker
///
use crate::types::*;
use log::{debug, info};

/**
 * A Ranker interface/trait, it's passed the DB Record and returns a 'rank' sorted list of same
 * no traits for collections yet in rust, so this is verbose to abstract
 **/
pub trait Ranker {
    fn rank(results: Vec<&mut Record>, keyword: String) -> Result<Vec<(u32, String)>>;
}

///Trivial Ranker, will just rank on hit count
pub struct WordCountRanker;

/**
 * This is a simplistic ranker, it just sums the total count of stems matched for each page
 **/
impl Ranker for WordCountRanker {
    fn rank(results: Vec<&mut Record>, keyword: String) -> Result<Vec<(u32, String)>> {
        info!(
            "WordCountRanker searching for {} in Results Set of Size {}",
            keyword,
            results.len(),
        );

        let mut result = results
            .iter()
            .map(|record| {
                (
                    record.stems.get(keyword.as_str()).unwrap_or(&0).clone(),
                    record.uri.to_string(),
                )
            })
            .collect::<Vec<(u32, String)>>();

        result.sort_by(|r1, r2| {
            debug!("r1={},r2={}", r1.0, r2.0);
            r2.0.cmp(&r1.0) //reverse order
        });

        Ok(result)
    }
}

#[cfg(tests)]
mod tests {
    use crate::{NaiveRanker, Ranker};

    #[test]
    fn test_word_ranker_default() {
        let ranker = WordCountRanker;
        let results = ranker.ranker(vec![Default::default()], "".into());
        assert_eq!(1, results.len());
        assert_eq!(results.uri, "".into())
    }

    #[test]
    fn test_word_ranker_basic() {
        let ranker = WordCountRanker;
        let results = ranker.ranker(
            vec![Record {
                id: "test",
                uri: "test",
                title: "test",
                stems: [("the", 2)].into_iter().clone().collect(),
            }],
            "".into(),
        );
        assert_eq!(1, results.len());
        assert_eq!(results.uri, "test".into())
    }

    #[test]
    fn test_word_ranker_empty() {
        let ranker = WordCountRanker;
        let results = ranker.ranker(vec![], "".into());
        assert_eq!(0, results.len())
    }
}

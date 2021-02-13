///! These tare the Ranker implementations, allowing for a algorithm to use R Where T:Ranker
///
use crate::types::*;
use log::{debug, info};

/**
 * A Ranker interface/trait, it's passed the DB Record and returns a 'rank' sorted list of same
 * no traits for collections yet in rust, so this is verbose to abstract
 **/
pub trait Ranker {
    fn rank(self, results: Vec<&mut Record>, keyword: String) -> Result<Vec<(u32, String)>>;
}

///Trivial Ranker, will just rank on hit count
#[derive(Copy)]
pub struct WordCountRanker;

impl Clone for WordCountRanker {
    fn clone(&self) -> Self {
        *self
    }
}

/**
 * This is a simplistic ranker, it just sums the total count of stems matched for each page
 **/
impl Ranker for WordCountRanker {
    fn rank(self, results: Vec<&mut Record>, keyword: String) -> Result<Vec<(u32, String)>> {
        info!(
            "WordCountRanker searching for {} in Results Set of Size {}",
            keyword,
            results.len(),
        );

        let mut result = results
            .iter()
            .map(|record| {
                (
                    *record.stems.get(keyword.as_str()).unwrap_or(&0),
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

#[cfg(test)]
mod tests {
    use crate::types::Record;
    use crate::{Ranker, WordCountRanker};

    #[test]
    fn test_word_ranker_basic() {
        let ranker = WordCountRanker;
        let results = ranker.rank(
            vec![&mut Record {
                id: "test".into(),
                uri: "test".into(),
                title: "test".into(),
                stems: [("the".into(), 2)].iter().cloned().collect(),
            }],
            "".into(),
        );
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(1, results.len());
        assert_eq!(results.get(0).unwrap(), &(0, "test".into()))
    }

    #[test]
    fn test_word_ranker_empty() {
        let ranker = WordCountRanker;
        let results = WordCountRanker.rank(vec![], "".into());
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(0, results.len())
    }
}

///! These tare the Ranker implementations, allowing for a algorithm to use R Where T:Ranker
///
use crate::types::*;

/**
 * A Ranker interface/trait, it's passed the DB Record and returns a 'rank' sorted list of same
 * no traits for collections yet in rust, so this is verbose to abstract
 **/
pub trait Ranker {
    fn rank(results: &Vec<Record>, keyword: String) -> Result<Vec<(usize, String)>>;
}

pub struct NaiveRanker;
pub struct WordCountRanker;

/**
 * This is a test ranker, it just orders by iter
 **/
impl Ranker for NaiveRanker {
    fn rank(results: &Vec<Record>, _keyword: String) -> Result<Vec<(usize, String)>> {
        Ok(results
            .iter()
            .enumerate()
            .map(|record_with_index| (record_with_index.0, record_with_index.1.uri.clone()))
            .collect())
    }
}

/**
 * This is a simplistic ranker, it just sums the total count of stems matched for each page
 **/
impl Ranker for WordCountRanker {
    fn rank(results: &Vec<Record>, keyword: String) -> Result<Vec<(usize, String)>> {
        Ok(results
            .iter()
            .map(|record| {
                record
                    .stems
                    .into_iter()
                    .filter(|w| w == &keyword)
                    //DOT sum how many stems match
                    .collect()
            })
            .collect())
    }
}

#[cfg(tests)]
mod tests {
    use crate::{NaiveRanker, Ranker};

    #[test]
    fn test_rank_one_rec() {
        let ranker = NaiveRanker;
        let results = ranker.ranker(vec![Default::default()], "".into());
        assert_eq!(1, results.len());
        assert_eq!(results.uri, "".into())
    }

    #[test]
    fn test_rank_empty() {
        let ranker = NaiveRanker;
        let results = ranker.ranker(vec![], "".into());
        assert_eq!(0, results.len())
    }
}

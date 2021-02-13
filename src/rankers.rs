///! These tare the Ranker implementations, allowing for a algorithm to use R Where T:Ranker
///
use crate::types::*;

/**
 * A Ranker interface/trait, it's passed the DB Record and returns a 'rank' sorted list of same
 **/
pub trait Ranker {
    fn rank(
        results: &Vec<Record>,
        keyword: String,
    ) -> Result<std::collections::HashMap<usize, String>>;

    fn rank1(results: &Vec<Record>, keyword: String) -> Result<Vec<(usize, String)>>;
}

pub struct NaiveWordCounterRanker;

/**
 * This is a simplistic ranker, it just sums the total count of stems matched for each page
 **/
impl Ranker for NaiveWordCounterRanker {
    fn rank(
        results: &Vec<Record>,
        _keyword: String,
    ) -> Result<std::collections::HashMap<usize, String>> {
        Ok(results
            .iter()
            .enumerate()
            .map(|record_with_index| (record_with_index.0, record_with_index.1.uri.clone()))
            .collect())
    }

    fn rank1(results: &Vec<Record>, keyword: String) -> Result<Vec<(usize, String)>> {
        Ok(results
            .iter()
            .enumerate()
            .map(|record_with_index| (record_with_index.0, record_with_index.1.uri.clone()))
            .collect())
    }
}

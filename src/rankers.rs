use crate::types::*;

/**
 * A Ranker interface/trait, it's passed the DB Record and returns a 'rank' sorted list of same
 **/
pub trait Ranker {
    fn rank(
        results: Vec<Record>,
        keyword: String,
    ) -> Result<std::collections::HashMap<String, String>>;
}

pub struct NaiveWordCounterRanker;

impl Ranker for NaiveWordCounterRanker {
    fn rank(
        results: Vec<Record>,
        keyword: String,
    ) -> Result<std::collections::HashMap<String, String>> {
        unimplemented!()
    }
}

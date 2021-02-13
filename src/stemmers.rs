use log::debug;
use porter_stemmer::stem;
use unicode_segmentation::UnicodeSegmentation;

pub trait IStemmer {
    /// Generate a based list of 'stems' from the article text
    ///
    /// # Arguments
    ///
    /// * `text` - the plaintext article from wikipedia
    fn istem(self, text: String) -> Vec<String>;
}

#[derive(Copy)]
pub struct SimplePorterStemmer {}

impl Clone for SimplePorterStemmer {
    fn clone(&self) -> Self {
        *self
    }
}

impl IStemmer for SimplePorterStemmer {
    /// Generate a porter_stemmer based list of 'stems' from the article text
    ///
    /// # Arguments
    ///
    /// * `text` - the plaintext article from wikipedia
    fn istem(self, text: String) -> Vec<String> {
        debug!("Stemming");
        let text = text.as_str().to_lowercase();
        let tokenised_sentence = text.unicode_words();
        tokenised_sentence.map(stem).collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::{IStemmer, SimplePorterStemmer};

    #[test]
    fn test_stemmer() {
        let stemmer = SimplePorterStemmer {};
        let sentence = " he ran she runs they run he is a runner".to_string();
        assert_eq!(
            10,
            stemmer.istem(sentence.clone()).len(),
            "{:?}",
            stemmer.istem(sentence)
        );

        let sentence = "".to_string();
        assert_eq!(
            0,
            stemmer.istem(sentence.clone()).len(),
            "{:?}",
            stemmer.istem(sentence)
        )
    }
}

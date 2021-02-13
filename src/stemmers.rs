use log::debug;
use porter_stemmer::stem;
use unicode_segmentation::UnicodeSegmentation;

pub trait IStemmer {
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
    fn istem(self, text: String) -> Vec<String> {
        debug!("Stemming");
        let text = text.as_str().to_lowercase();
        let tokenised_sentence = text.unicode_words();
        tokenised_sentence.map(stem).collect::<Vec<String>>()
    }
}

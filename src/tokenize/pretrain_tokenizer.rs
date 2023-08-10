use anyhow::{anyhow, Result};
use std::collections::HashMap;
use tokenizers::tokenizer::Tokenizer;
use unicode_segmentation::UnicodeSegmentation;
use std::fmt::Debug;
use std::io::Write;
use std::marker::Copy;
use serde::Serialize;
use serde::Deserialize;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::tokenize::Tokenize;

// pub(crate) fn tokenize(s: &str) -> impl Iterator<Item = &str> {
//     s.split_word_bounds().filter(|w| {
//         for c in w.chars() {
//             if !c.is_whitespace() {
//                 return true;
//             }
//         }
//         false
//     })
// }

#[derive(Debug, Clone)]
pub struct PretrainedTokenizer {
    pub tokenizer: Tokenizer,
}

impl PretrainedTokenizer {
    pub fn new(name: &str) -> Self {
        let tokenizer = Tokenizer::from_pretrained(name, None)
            .map_err(|err| anyhow!("Failed to load pretrained tokenizer {} - {}", name, err))
            .unwrap();

        PretrainedTokenizer { tokenizer }
    }
}

impl<E> Tokenize<E> for PretrainedTokenizer 
where
    E: Eq + serde::Serialize + Copy + Debug + TryFrom<u32>,
{
    fn build(&mut self, text: &str) {
        // do nothing (pretrained tokenizer is already built)
    }

    fn tokenize(&mut self, text: &str) -> Vec<E> {
        // let tokenized_text: Vec<_> = text
        //     .split_whitespace()
        //     .map(|x| E::try_from(self.tokenizer.token_to_id(x)
        //     .unwrap_or_default())
        //     .unwrap_or_else(|_| panic!("Err!!!")))
        //     .collect();
        // tokenized_text
        // self.tokenizer.encode(text, false).unwrap_or_else(|_| panic!("Err!!!"))
        let output = self.tokenizer.encode(text, true);
        let bindings = output.expect("REASON");//.get_ids();
        let ids = bindings.get_ids();
        let converted_values: Vec<E> = ids.iter().map(|&num| num.try_into().unwrap_or_else(|_| panic!("Err!!!"))).collect();
        converted_values
    }

    fn get_count(&self) -> usize {
        self.tokenizer.get_vocab_size(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenize::{PretrainedTokenizer, Tokenize};
    use std::convert::TryFrom;

    #[test]
    fn test_gpt2_tokenizer() {
        let mut token_index: Box<dyn Tokenize<u16>> = Box::new(PretrainedTokenizer::new("gpt2"));
        
        println!("vocab size: {:?}", token_index.get_count());
        println!("{:?}", token_index.tokenize("hello"));

        // let mut token_index2: PretrainedTokenizer = PretrainedTokenizer::new("gpt2");
        // assert_eq!(
        //     token_index2.tokenize("hello"),
        //     vec! {usize::try_from(
        //     token_index2.tokenizer.token_to_id("hello").unwrap())
        //     .unwrap()}
        // );

        assert_eq!(token_index.get_count(), 50257);

        assert_eq!(token_index.tokenize("hello world"), [31373, 995]);
    }
}

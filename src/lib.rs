use pyo3::prelude::*;
use std::borrow::Cow;

/// returns Sørensen-Dice coefficient for two words

#[pymodule]
fn sdcoeff(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "coefficient")] // name of the Python function in module sdcoeff

    fn sd_coeff_py(_py: Python, word_a: &str, word_b: &str) -> PyResult<f32> {
        let result = sd_coeff(&word_a, &word_b);
        Ok(result)
    }

    Ok(())
}

// logic implemented in Rust

fn sd_coeff(word_a: &str, word_b: &str) -> f32 {
    
    // calculate Sørensen–Dice coefficient

    let mut coeff = 0.0_f32;

    if (word_a.len() < 2 ) | (word_b.len() < 2  ) { // return zero if any of the words doesn't have at least one bigram

        coeff = 0.0;
        
    }

    else {

        let bigrams_a = get_bigrams(word_a);
        let bigrams_b = get_bigrams(word_b);
    
        let common = compare_bigrams(&bigrams_a, &bigrams_b);

        coeff = (2.0 * common as f32) / (bigrams_a.len() + bigrams_b.len()) as f32;

    }
    
    return coeff;
    
}

fn get_bigrams(word: &str) -> Vec<Cow<str>> {
    
    // split a word into bigrams

    let mut bigrams: Vec<_> = Vec::new();
    
    let word_vec = word.chars().collect::<Vec<_>>();

    for i in 0..(word_vec.len()-1) {

        let bigram = &word_vec[i..i+2];

        let mut new_bigram = "".to_string();

        for item in bigram.iter() {
            new_bigram.push(*item);
        }

        bigrams.push(Cow::Owned(new_bigram));

    }

    return bigrams;

}

fn compare_bigrams(bigrams_a: &[Cow<str>], bigrams_b: &[Cow<str>]) -> u8 {
    
    // count how many bigrams are common between two words
    
    let mut common_count: u8 = 0;
        
    for item_a in bigrams_a.iter() {
            
        if bigrams_b.contains(item_a) {
            common_count += 1;        
        }    
    }
        
    return common_count;
}

















#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

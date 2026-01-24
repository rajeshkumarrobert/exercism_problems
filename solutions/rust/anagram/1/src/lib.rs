use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut given_word = word.to_lowercase().chars().collect::<Vec<char>>();
    given_word.sort();
    let mut res = HashSet::new();
    for slice in possible_anagrams{
        if word.to_lowercase() != slice.to_lowercase(){
            let mut new_word = slice.to_lowercase().chars().collect::<Vec<char>>();
            new_word.sort();
            if given_word.eq(&new_word){
                res.insert(*slice);
            }
        }
    }
    res
}

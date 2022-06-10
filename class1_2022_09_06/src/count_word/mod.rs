
use regex::Regex;

pub enum SearchType {
    Normal,
    Regex
}

pub fn count_words(string_find: &str, string_given: &str, type_search: SearchType) -> u32 {
   
    match type_search {
        SearchType::Normal => count_words_normal(string_find, string_given),
        SearchType::Regex => count_words_regex(string_find, string_given)
    }
}

fn count_words_normal(string_find: &str, string_given: &str) ->u32 {
    string_given.to_string().to_lowercase().matches(string_find).count() as u32
}

fn count_words_regex(string_find: &str, string_given: &str) ->u32 {
    let regex: Regex = Regex::new(string_find).unwrap();
    regex.find_iter(string_given).count() as u32
}


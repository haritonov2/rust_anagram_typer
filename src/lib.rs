use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();
    let input_letters: Vec<char> = lowercase_word.chars().collect();
    let mut anagrams_result = HashSet::new();

    'outer: for anagram in possible_anagrams {
        if word.len() != anagram.len() {
            continue;
        }

        let lowercase_anagram = anagram.to_lowercase();

        if lowercase_anagram == lowercase_word {
            continue;
        }

        let mut anagram_letters: HashMap<char, i32> = HashMap::new();

        for letter in lowercase_anagram.chars() {
            *anagram_letters.entry(letter).or_insert(0) += 1;
        }

        for letter in &input_letters {
            let letter_count = anagram_letters.entry(*letter).or_insert(0);

            if *letter_count > 0 {
                *letter_count -= 1;
            } else {
                continue 'outer;
            }
        }

        anagrams_result.insert(*anagram);
    }

    anagrams_result
}

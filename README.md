# Rust Anagram Typer

This project is a Rust implementation for finding anagrams of a given target word from a list of candidates. It handles case-insensitive matching, works with Unicode characters, and ensures that a word is not considered an anagram of itself.

## Usage

To find anagrams, call the `anagrams_for` function, providing a target word and a list of candidate words.

Example:
```rust
use rust_anagram_typer::anagrams_for;
use std::collections::HashSet;

let word = "listen";
let candidates = &["enlists", "google", "inlets", "banana"];
let anagrams = anagrams_for(word, candidates);

let expected: HashSet<&str> = HashSet::from_iter(["inlets"]);
assert_eq!(anagrams, expected);
```




// * think of match in a for loop and add the results to a string

/*
    The code is pretty much done lmao this took way less time than I thought.
    todo: Add support for all the punctuation cited at this link:
    / https://www.electronics-notes.com/articles/ham_radio/morse_code/characters-table-chart.php
*/

use std::collections::HashMap;

const AMOUNT_OF_CHARS: usize = 41;
type HashArray = [&'static str; AMOUNT_OF_CHARS];

const CHARS: HashArray = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "g", "h", "i",
    "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", " ", "!",
    ".", ",", "?",
];

const MORSE: HashArray = [
    "-----", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.", ".-",
    "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.",
    "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..", "/",
    "-.-.--", ".-.-.-", "--..--", "..__..",
];

type DictHash = HashMap<&'static str, &'static str>;

fn create_hash_map(key_array: HashArray, value_array: HashArray) -> DictHash {
    let mut hash = HashMap::new();

    for i in 0..AMOUNT_OF_CHARS {
        hash.insert(key_array[i], value_array[i]);
    }

    return hash;
}

fn encode(str: &str) -> String {
    let char_hash = create_hash_map(CHARS, MORSE);
    let mut encoded_morse = String::new();
    let mut unknown_chars: String = String::new();

    for letter in str.to_lowercase().chars() {
        let hash_key = char_hash.get(letter.to_string().as_str());
        encoded_morse += format!("{} ", hash_key.unwrap_or(&"")).as_str();

        if let None = hash_key {
            unknown_chars += &letter.to_string();
        }
    }

    if !unknown_chars.is_empty() {
        println!(
            "The following characters were removed during encoding as they were unknown:\n{}\n",
            unknown_chars
        );
    };

    if encoded_morse.trim().is_empty() {
        return String::from("All characters from the inputted string were unknown.");
    }

    return String::from("Encoded String: ") + &encoded_morse;
}

fn decode(str: &str) -> String {
    let morse_hash = create_hash_map(MORSE, CHARS);
    let mut encoded_morse = String::new();
    let mut unknown_morse: String = String::new();

    // In case I can't loop over a Splitter just add a collect
    for morse_c in str.replace("_", "-").split(" ") {
        let hash_key = morse_hash.get(morse_c.to_string().as_str());
        encoded_morse += &hash_key.unwrap_or(&"");

        if let None = hash_key {
            unknown_morse += morse_c;
        }
    }

    if !unknown_morse.is_empty() {
        println!(
            "The following morse code was removed during decoding as they were unknown:\n{}\n",
            unknown_morse
        );
    }

    if encoded_morse.trim().is_empty() {
        return String::from("All characters from the inputted morse code are unknown.");
    }

    return String::from("Decoded String: ") + &encoded_morse;
}

fn main() {
    let char_str: &str = "BHENCHTTTD";
    let morse_str: &str = "_... .... . _. _._. .... _ _ _ _..";
    println!("{}\n{}", encode(char_str), decode(morse_str));
}

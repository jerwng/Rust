fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut most_frequent_word = words[0];
    let mut most_frequent_word_count = 0;

    for word in &words {
        let mut word_frequency_counter = 0;

        for other_word in &words {
            if *word == *other_word {
                word_frequency_counter += 1;
            }
        }

        if word_frequency_counter > most_frequent_word_count {
            most_frequent_word = word;
            most_frequent_word_count = word_frequency_counter;
        }
    }
 
    return (String::from(most_frequent_word), most_frequent_word_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
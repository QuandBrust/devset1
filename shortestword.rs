fn find_shortest_word(s: &str) -> Option<&str> {
    let mut shortest_word: Option<&str> = None;
    let mut shortest_length: usize = usize::MAX;

    for word in s.split_whitespace() {
        let word_length = word.len();
        if word_length < shortest_length {
            shortest_length = word_length;
            shortest_word = Some(word);
        }
    }

    shortest_word
}

fn main() {
    let input_string = "The quick brown fox jumps over the lazy dog";
    match find_shortest_word(input_string) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found in the input string"),
    }
}
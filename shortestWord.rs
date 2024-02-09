fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|w| w.len())
}

fn main() {
    let sentence = "This is a sample sentence";
    let shortest = shortest_word(sentence);

    match shortest {
        Some(word) => println!("The shortest word is: {}", word),
        None => println!("No words found"),
    }
}
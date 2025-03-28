
fn find_nemo(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();

    if let Some(index) = words.iter().position(|&word| word == "Nemo") {
        format!("I found Nemo at {}!", index +1)
    } else {
        "I can't find Nemo :(".to_string()
    }
}

fn main() {
    println!("{}", find_nemo("I am finding Nemo !"));
}
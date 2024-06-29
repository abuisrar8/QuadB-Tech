use std::io::{self,Write};

fn size_of_min_word(sentence: &str) -> usize {
    let mut min_size = usize::MAX;

    for word in sentence.split_whitespace() {
        if word.len() < min_size{
            min_size = word.len();
        }
    }
    min_size
}

fn main() {
    let mut str =String::new();
    print!("enter a sentence : ");
    io::stdout().flush();
    io::stdin().read_line(&mut str);
    str=str.trim().to_string();
    println!("Size of the smallest word is {}",size_of_min_word(&str));

}
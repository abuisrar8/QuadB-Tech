fn longest_common_prefix(str_: Vec<&str>)-> String {
    if str_.is_empty(){return "".to_string();}
    let mut sorted_str = str_.clone();
    sorted_str.sort();

    let first_word = sorted_str[0];
    let last_word = sorted_str[sorted_str.len()-1];

    let mut i =0;
    while i<first_word.len()&&i<last_word.len()&&first_word.chars().nth(i)==last_word.chars().nth(i){
        i+=1;
    } 
    first_word[0..i].to_string()
}

fn main() {
    let str_ = vec!["iaiaxuw","iaian","iaiac","iaia"];
    println!("{}", format!("the longest commaon prefix is : {}" ,longest_common_prefix(str_)));
}
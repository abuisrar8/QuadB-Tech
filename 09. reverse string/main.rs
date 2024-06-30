use std::io::{self,Write};
fn reverse_string(s:&str)->String{
    s.chars().rev().collect()
}
fn main(){
    let mut str =String::new();
    print!("enter a sentence : ");
    io::stdout().flush();
    io::stdin().read_line(&mut str);
    str=str.trim().to_string();

    println!("{}",reverse_string(&str));
}
fn check_palindrome(str: &str) -> bool {
    let chars: Vec<char> = str.chars().collect();
    let mut i = 0;
    let mut j = chars.len() - 1;

    while i <= j {
        if chars[i] != chars[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}

fn main() {
    let str = "abababababa";

    if check_palindrome(str) {
        println!("{} is a palindrome :) ", str);
    } else {
        println!("{} is not a palindrome :( ", str);
    }
}

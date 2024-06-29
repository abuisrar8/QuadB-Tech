use std::io;

fn check_prime(n:i32) -> bool {
    if n <=1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as i32){
        if n%i == 0{
            return false;
        }
    }
    true
}

fn main(){
    println!("Enter a number: ");
    let mut userinp = String::new();
    io::stdin().read_line(&mut userinp);
    let n :i32 = userinp.trim().parse().expect("invalid no. ");

    if check_prime(n){
        println!("{} isa prime no. ",n);

    } else {
        println!("{} is mot a prime no. ",n);
    }
}
fn median_of_sorted_array(arr:&[i32])->f64 {
    let n = arr.len();
    if n==0 {
        return 0.0;
    }
    if n%2 == 0{
    (arr[n/2-1] + arr[n/2]) as f64 /2.0
    } else {
        arr[n/2] as f64
    }
}

fn main(){
    let arr = [1,2,2,2,3,3,4,5,5,6,7,8,6,10];
    println!("median : {}",median_of_sorted_array(&arr));
}
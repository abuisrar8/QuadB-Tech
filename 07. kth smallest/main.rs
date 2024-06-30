fn k_smallest(arr:&mut [i32],k:usize) -> i32{
    let n = arr.len();
    if k>n{
        return -1;
    }
    arr.sort();
    let mut unique_count = 1;
    for i in 1..n {
        if arr[i]!= arr[i-1]{
            unique_count +=1;
            if unique_count == k{
                return arr[i];
            }
        }
    }
    -2
}

fn main(){
    let mut arr = vec![1,3,5,4,3,7,3,1,7];
    let k =2;

    let option = k_smallest(&mut arr,k) ;
    if option == -1 {
        println!("{} is not in range", k);

    } else if option ==-2{
        println!("There are less than {} unique elements", k)
    }
    else {
        println!("The {}-th unique smallest element is {}", k, option)
    }
}
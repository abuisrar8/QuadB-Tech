use std::io;

fn merge_two_sorted_array(arr1:&[i32],arr2:&[i32])-> Vec<i32>{
    let n = arr1.len();
    let m = arr2.len();
    let mut merged = Vec::with_capacity(n+m);
    let mut i =0;
    let mut j =0;

    while i<n && j<m {
        if arr1[i]>arr2[j] {
            merged.push(arr2[j]);
            j+=1;
        }else{
            merged.push(arr1[i]);
            i+=1;
        }
    }
    while i<n{
        merged.push(arr1[i]);
        i+=1;
    }
    while j<m{
        merged.push(arr2[j]);
        j+=1;
    }
    merged
}

fn main(){
    let mut input = String::new();
    println!("Enter size of 1st array: ");
    io::stdin().read_line(&mut input).expect("failed..");
    let n: usize = input.trim().parse().expect("failed..");
    input.clear();
    let mut input = String::new();
    println!("Enter size of 2nd array: ");
    io::stdin().read_line(&mut input).expect("failed..");
    let m: usize = input.trim().parse().expect("failed..");
    input.clear();

    let mut arr1 = Vec::with_capacity(n);
    let mut arr2 = Vec::with_capacity(m);

    println!("Enter elemnets of 1st array: ");
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("failed..");
        arr1.push(input.trim().parse().expect("failed.."));
    }
    println!("Enter elemnets of 2nd array: ");
    for _ in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).expect("failed..");
        arr2.push(input.trim().parse().expect("failed"));
    }

    arr1.sort();
    arr2.sort();

    let merged = merge_two_sorted_array(&arr1,&arr2);
    println!("merged.. ");
    for i in merged{
        print!("{} ",i);
    }

}
fn _1st_occurence(arr:&[i32],find:i32)-> i32{
    for(i,&num) in arr.iter().enumerate() {
        if num == find {
            return i as i32;
        }
    }
    -1
}
fn main(){
    let arr = [1,2,2,2,3,3,4,5,5,6];
    let find = 5;

    let pos = _1st_occurence(&arr,find);
    if pos == -1{
        println!("{} not presemt",find);

    }
    else {
        println!("1st {} present at {} th position.",find,pos);
    }
   
}
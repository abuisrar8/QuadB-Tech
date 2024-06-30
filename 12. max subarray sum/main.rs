fn kadane_max_sunarray_sum(arr:&[i32]) -> i32{
    let mut _max =i32::MIN;
    let mut max_pos = 0;

    for &i in arr {
        max_pos +=i;
        if _max<max_pos{ _max=max_pos;}
        if max_pos<0{ max_pos=0;}

    }
    _max
}

fn main() {
    let arr = [-2,1,-3,-4,-1,2,1,-5,4];
    println!("max subarray sum is : {}",kadane_max_sunarray_sum(&arr));
}
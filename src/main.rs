fn main() {
/*     let mut s = String::from("foo");
    s.push_str(" world");

    println!("{}",s);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}",s3); */
    
    let numbers = vec![2,7,11,15];
    println!("{:?}",two_sum(numbers, 9));
}
/* Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order. */
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut sum=0;
    let mut i_skip = 0; 
    for (i,num ) in nums.iter().enumerate().skip(i_skip) {
        for (j,num1) in nums.iter().enumerate().skip(i_skip + 1){
            if(target == num + num1 ){
                return vec![i.try_into().unwrap(),j.try_into().unwrap()];
            }
        }
        i_skip += 1;
    
    }

    
    return vec![];
    
 }

 pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let vec_len = nums.len();
    for i in 0..vec_len {
        for y in i + 1..vec_len {
            if nums[i] + nums[y] == target {
                return vec![i as i32, y as i32];
            }
        }
    }
    vec![]
}


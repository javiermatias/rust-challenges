/* Given a non-empty array of integers nums, every element appears twice except for one. 
Find that single one.
You must implement a solution with a linear runtime complexity and use only constant extra space.

Example 1:

Input: nums = [2,2,1]
Output: 1 */


fn main() {
    let nums = [4,1,2,1,2];

    println!("{}",single_number(nums.to_vec()));
 
}

pub fn single_number(nums: Vec<i32>) -> i32 {
        
        let mut result = 0;
        for num in nums{
           result ^= num;
        }
        return result;
}
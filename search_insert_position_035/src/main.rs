// https://leetcode.com/problems/search-insert-position

pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let length = nums.len() - 1;
        let mut left: usize = 0;
        let mut right = length;

        while left <= right {
            let mid: usize = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid]< target {
                if mid >= length || nums[mid+1] > target {
                    return (mid+1) as i32;
                }
                else {
                    left = mid + 1;
                }
            } else {
                if mid == 0  {
                    return 0;
                }
                else {
                    right = mid - 1;
                }  
            }
        }
        0
    }
}

fn main() {
    let nums = vec![1,3,5,6];
    let target = 0;
    let index = Solution::search_insert(nums, target);
    println!("index {}", index);
}
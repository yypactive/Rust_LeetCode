// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

pub struct Solution;

impl Solution {
    pub fn binary_search(nums: &Vec<i32>, target: i32, begin_index: usize, end_index: usize) -> usize {
        let mut left = begin_index;
        let mut right = end_index;

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return mid;
            } else if nums[mid]< target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        0
    }

    pub fn two_sum_ii_167(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let sorted_nums = nums;
        let mut first_index:usize = 0;
        let mut second_index:usize = 0;
        let length = sorted_nums.len();
        let max_number = sorted_nums[length-1];
        for index in 0..length {
            let first_num = sorted_nums[index];
            if first_num >= target - max_number && first_num <= target / 2 {
                second_index = Solution::binary_search(&sorted_nums, target - first_num, index+1, length-1);
                if second_index > 0 {
                    // find
                    first_index = index;
                    break;
                }
            }
        }
        let result = vec![(first_index as i32)+1, (second_index as i32)+1];
        result        
    }
}


fn main() {
    let nums = vec![2,7,11,15];
    let target = 9;
    let result = Solution::two_sum_ii_167(nums, target);
    println!("{}, {}", result[0], result[1]);
}
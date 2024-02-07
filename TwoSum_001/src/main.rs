// https://leetcode.com/problems/two-sum/

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

    pub fn two_sum_001(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
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
        let first_num = sorted_nums[first_index];
        let second_num = sorted_nums[second_index];
        let mut find = 0;
        if first_num == second_num {
            for index in 0..length {
                let num = nums[index];
                if num == first_num {
                    if find == 0 {
                        first_index = index;
                        find += 1;
                    }
                    else if find == 1 {
                        second_index = index;
                        break;
                    }
                }
            }
        }
        else {
            for index in 0..length {
                let num = nums[index];
                if num == first_num {
                    first_index = index;
                    find += 1;
                }
                else if num == second_num  {
                    second_index = index;
                    find += 1;
                }
                if find >= 2 {
                    break;
                }
            }
        }
        let result = vec![first_index as i32, second_index as i32];
        result        
    }
}



fn main() {
    let nums = vec![2,7,11,15];
    let target = 9;
    let result = Solution::two_sum_001(nums, target);
    println!("{}, {}", result[0], result[1]);
}
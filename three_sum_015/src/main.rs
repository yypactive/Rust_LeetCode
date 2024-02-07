// https://leetcode.com/problems/search-insert-position

pub struct Solution;

impl Solution {
    pub fn binary_search(nums: &Vec<i32>, begin_index: usize, end_index:usize, target: i32) -> i32 {
        let mut left: usize = begin_index;
        let mut right: usize = end_index;

        while left <= right {
            let mid: usize = left + (right - left) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid - 1; 
            }
        }
        0
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32> > {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        let sorted_nums_length = sorted_nums.len();
        let mut res:Vec<Vec<i32> > = vec![];
        for i in 0..sorted_nums_length-2 {
            let first_num = sorted_nums[i];
            if i > 0 && first_num == sorted_nums[i-1] {
                continue;
            }
            let left: usize = i+1;
            for j in left..sorted_nums_length-1 {
                let second_num = sorted_nums[j];
                if j > left && second_num == sorted_nums[j-1] {
                    continue;
                }
                let third_index = Solution::binary_search(&sorted_nums, j+1, sorted_nums_length-1, -first_num-second_num);
                if third_index > 0 {
                    let third_num = sorted_nums[third_index as usize];
                    res.push(vec![first_num, second_num, third_num]);
                }
            }
        }
        res
    }
}

fn main() {
    let nums = vec![-1,0,1,2,-1,-4];
    let output:Vec<Vec<i32> > = Solution::three_sum(nums);
    let output_str:Vec<String> = output.iter().map(|inner_vec| format!("{:?}", inner_vec)).collect();
    println!("output {:?}", output_str);
}
use std::collections::HashMap;

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(2);
        let mut i = 0;
        let mut j = i + 1;
        let mut found = false;
        while i != nums.len() - 1 {
            j = i + 1;
            while j != nums.len() {
                if nums[i] + nums[j] == target {
                    found = true;
                    break;
                }
                j += 1;
            }
            if found {
                break;
            }
            i += 1;
        }
        if found {
            result.push(i as i32);
            result.push(j as i32);
        }
        return result;
    }

    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(2);
        let mut num_idx = HashMap::new();
        let mut idx = 0;
        for num in nums {
            let another_num = target - num;
            match num_idx.get_key_value(&another_num) {
                None => { num_idx.insert(num, idx); }
                Some(kv) => { 
                    result.push(*kv.1);
                    result.push(idx);
                }
            };
            idx += 1;
        }
        return result;
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    println!("result: {:?}", Solution::two_sum_2(nums, target));
}

struct Solution {}

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let num_2_digit_vec = |mut num: i32| -> Vec<i32> {
            let mut digit_vec = Vec::new();
            loop {
                digit_vec.push(num % 10);
                num /= 10;
                if num == 0 {
                    break;
                }
            }
            return digit_vec;
        };

        let mut position_num_count_map: std::collections::HashMap<i32, [i32; 10]> = std::collections::HashMap::new();

        let mut sum = 0_i64;
        let mut count = 0;
        for num in nums {
            let digit_vec = num_2_digit_vec(num);
            let mut position = 0;
            for digit in digit_vec {
                if let Some(num_count) = position_num_count_map.get_mut(&position) {
                    num_count[digit as usize] += 1;
                    sum += count - num_count[digit as usize] as i64 + 1;
                } else {
                    let mut new_num_count = [0; 10];
                    new_num_count[digit as usize] = 1;
                    position_num_count_map.insert(position, new_num_count);
                }
                position += 1;
            }
            count += 1;
        }

        return sum;
    }
}

fn main() {
    let nums = vec![13,34,45];
    println!(
        "sum_digit_differences: {}",
        Solution::sum_digit_differences(nums)
    );
}

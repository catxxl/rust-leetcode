use std::cmp::max;
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let chars = s.as_bytes();
        let mut left = 0;
        let mut right = left;
        let str_len = s.len();
        let mut max_len = 1_i32;
        let mut char_set = HashSet::new();
        char_set.insert(chars[left]);
        loop {
            if right + 1 < str_len {
                let next_right = chars[right + 1];
                if char_set.contains(&next_right) {
                    loop {
                        char_set.remove(&chars[left]);
                        left += 1;
                        if next_right == chars[left - 1] {
                            break;
                        }
                    }
                }
                char_set.insert(next_right);
                max_len = max(max_len, char_set.len() as i32);
                right += 1;
            } else {
                break;
            }
        }

        return max_len;
    }
}
fn main() {
    let s = String::from("pwwkew");
    println!(
        "length_of_longest_substring: {}",
        Solution::length_of_longest_substring(s)
    );
}

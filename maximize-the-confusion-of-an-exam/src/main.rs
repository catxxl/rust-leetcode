struct Solution {}

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut t_sums: Vec<i32> = std::iter::repeat(0).take(answer_key.len()).collect();
        let mut idx = 0;
        for ch in answer_key.chars() {
            if idx != 0 {
                t_sums[idx] = t_sums[idx - 1];
            }
            if ch == 'T' {
                t_sums[idx] += 1;
            }
            idx += 1;
        }

        let mut answer: i32 = 1;
        let mut left = 0;
        let mut right = left;
        while right < answer_key.len() {
            if left < right {
                let mut t_sum = t_sums[right];
                if left != 0 {
                    t_sum -= t_sums[left - 1];
                }
                let f_sum = (right - left + 1) as i32 - t_sum;
                if std::cmp::min(t_sum, f_sum) <= k {
                    answer = std::cmp::max(answer, (right - left + 1) as i32);
                } else {
                    left += 1;
                }
            }
            right += 1;
        }

        return answer;
    }

    pub fn max_consecutive_answers_1(answer_key: String, k: i32) -> i32 {
        let mut t_sums: Vec<i32> = std::iter::repeat(0).take(answer_key.len()).collect();
        let mut answer: i32 = 1;
        let mut left = 0;
        let mut right = left;
        let bytes = answer_key.as_bytes();
        while right < answer_key.len() {
            if right == 0 {
                t_sums[0] = match bytes[right] {
                    b'T' => 1,
                    _ => 0,
                };
            } else {
                t_sums[right] = t_sums[right - 1];
                if bytes[right] == b'T' {
                    t_sums[right] += 1;
                }
            }
            if left < right {
                let mut t_sum = t_sums[right];
                if left != 0 {
                    t_sum -= t_sums[left - 1];
                }
                let f_sum = (right - left + 1) as i32 - t_sum;
                if std::cmp::min(t_sum, f_sum) <= k {
                    answer = std::cmp::max(answer, (right - left + 1) as i32);
                } else {
                    left += 1;
                }
            }
            right += 1;
        }

        return answer;
    }
}

#[test]
fn it_works() {
    assert_eq!(
        4,
        Solution::max_consecutive_answers_1("TTFF".to_string(), 2)
    );
    assert_eq!(
        3,
        Solution::max_consecutive_answers_1("TFFT".to_string(), 1)
    );
    assert_eq!(
        5,
        Solution::max_consecutive_answers_1("TTFTTFTT".to_string(), 1)
    );
}

fn main() {
    println!(
        "max_consecutive_answers: {}",
        Solution::max_consecutive_answers_1("TTFTTFTT".to_string(), 1)
    );
}

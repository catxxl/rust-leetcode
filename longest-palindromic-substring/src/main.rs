struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut result: &str = &s[0..=0];
        let mut max_len = 1;

        let bytes = s.as_bytes();
        let is_palindrome = |mut start, mut end| {
            while start <= end {
                if bytes[start] != bytes[end] {
                    return false;
                }
                start += 1;
                end -= 1;
            }
            return true;
        };

        for i in 1..=s.len() - 1 {
            if i > max_len - 1 {
                for j in 0..i - max_len + 1 {
                    if is_palindrome(j, i) {
                        result = &s[j..=i];
                        max_len = i - j + 1;
                        break;
                    }
                }
            }
        }

        return result.to_string();
    }

    pub fn longest_palindrome_1(s: String) -> String {
        let mut result = &s[0..=0];
        let mut max_len = 1;

        let bytes = s.as_bytes();
        let mut starts: Vec<usize> = (0..s.len()).collect();
        let is_palindrome = |s1, e1, s2, e2| {
            let mut i = s1;
            let mut j = e2;
            while i <= e1 && j >= s2 {
                if bytes[i] != bytes[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            return true;
        };

        let is_palindrome_1 = |mut start, mut end| {
            while start <= end {
                if bytes[start] != bytes[end] {
                    return false;
                }
                start += 1;
                end -= 1;
            }
            return true;
        };

        for i in 1..=s.len() - 1 {
            for j in (0..=i - 1).rev() {
                let start = starts[j];
                if start >= i - j && i <= s.len() - 1 {
                    if j - start + 1 + 2 * (i - j) > max_len
                        && is_palindrome(start - (i - j), start - 1, j + 1, i)
                    {
                        starts[i] = start - (i - j);
                        max_len = j - start + 1 + 2 * (i - j);
                        result = &s[start - (i - j)..=i];
                    }
                }
                if i - start + 1 > max_len && is_palindrome_1(start, i) {
                    starts[i] = start;
                    max_len = i - start + 1;
                    result = &s[start..=i];
                }
            }
        }

        return result.to_string();
    }
}

#[test]
fn it_works() {
    assert_eq!(
        "aaabaaa",
        Solution::longest_palindrome_1("aaaabaaa".to_string())
    );
}

fn main() {
    println!("Hello, world!");
}

struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let find_median_of_sorted_array = |nums: Vec<i32>| -> f64 {
            let len = nums.len();
            match len % 2 {
                1 => nums[len / 2] as f64,
                _ => (nums[len / 2 - 1] + nums[len / 2]) as f64 / 2.0,
            }
        };

        // no overlap
        if nums1.is_empty() {
            return find_median_of_sorted_array(nums2);
        }
        if nums2.is_empty() {
            return find_median_of_sorted_array(nums1);
        }

        let mut nums: Vec<i32> = nums1.iter().chain(nums2.iter()).cloned().collect();
        nums.sort();
        return find_median_of_sorted_array(nums);
    }

    pub fn find_median_sorted_arrays_1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let find_median_of_sorted_array = |nums: Vec<i32>| -> f64 {
            let len = nums.len();
            match len % 2 {
                1 => nums[len / 2] as f64,
                _ => (nums[len / 2 - 1] + nums[len / 2]) as f64 / 2.0,
            }
        };

        // no overlap
        if nums1.is_empty() {
            return find_median_of_sorted_array(nums2);
        }
        if nums2.is_empty() {
            return find_median_of_sorted_array(nums1);
        }

        let len1 = nums1.len();
        let len2 = nums2.len();
        let len = len1 + len2;
        let k = (len1 + len2) / 2 + 1;
        let mut idx1 = 0;
        let mut idx2 = 0;
        let mut count = 0;
        let mut number_k = 0;
        let mut number_k_1 = 0;
        loop {
            if count == k {
                break;
            }
            number_k_1 = number_k;
            if idx2 == len2 || (idx1 != len1 && nums1[idx1] <= nums2[idx2]) {
                number_k = nums1[idx1];
                idx1 += 1;
            } else if idx1 == len1 || (idx2 != len2 && nums1[idx1] > nums2[idx2]) {
                number_k = nums2[idx2];
                idx2 += 1;
            }
            count += 1;
        }
        match len % 2 {
            0 => {
                return (number_k + number_k_1) as f64 / 2.0;
            }
            _ => {
                return number_k as f64;
            }
        }
    }

    // it's a little complex to update the max number(s)
    pub fn find_median_sorted_arrays_2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let find_median_of_sorted_array = |nums: Vec<i32>| -> f64 {
            let len = nums.len();
            match len % 2 {
                1 => nums[len / 2] as f64,
                _ => (nums[len / 2 - 1] + nums[len / 2]) as f64 / 2.0,
            }
        };

        // no overlap
        if nums1.is_empty() {
            return find_median_of_sorted_array(nums2);
        }
        if nums2.is_empty() {
            return find_median_of_sorted_array(nums1);
        }

        let len1 = nums1.len();
        let len2 = nums2.len();
        let len = len1 + len2;
        let mut k = (len1 + len2) / 2 + 1;
        let mut start1 = 0;
        let mut start2 = 0;
        let mut number_k = -1000001;
        let mut number_k_1 = -1000001;
        loop {
            // println!("k: {}, start1: {}, start2: {}", k, start1 + 1, start2 + 1);
            if k == 1 {
                if start1 == len1 {
                    number_k_1 = number_k;
                    number_k = std::cmp::max(number_k, nums2[start2]);
                } else if start2 == len2 {
                    number_k_1 = number_k;
                    number_k = std::cmp::max(number_k, nums1[start1]); 
                } else {
                    number_k_1 = number_k;
                    number_k = std::cmp::max(number_k, std::cmp::min(nums1[start1], nums2[start2]));
                }
                break;
            }
            if start1 == len1 {
                let new_big = nums2[start2 + k - 1];
                let new_little = nums2[start2 + k - 2];
                if new_big > number_k {
                    number_k_1 = std::cmp::max(number_k, new_little);
                    number_k = new_big;
                } else if new_big == number_k {
                    number_k_1 = number_k;
                } else {
                    number_k_1 = std::cmp::max(number_k_1, new_big);
                }
                break;
            } else if start2 == len2 {
                let new_big = nums1[start1 + k - 1];
                let new_little = nums1[start1 + k - 2];
                if new_big > number_k {
                    number_k_1 = std::cmp::max(number_k, new_little);
                    number_k = new_big;
                } else if new_big == number_k {
                    number_k_1 = number_k;
                } else {
                    number_k_1 = std::cmp::max(number_k_1, new_big);
                }
                break;
            }
            let end1 = std::cmp::min(start1 + k / 2 - 1, len1 - 1);
            let end2 = std::cmp::min(start2 + k / 2 - 1, len2 - 1);
            if nums1[end1] <= nums2[end2] {
                number_k_1 = number_k;
                number_k = std::cmp::max(number_k, nums1[end1]);
                k -= end1 - start1 + 1;
                start1 = end1 + 1;
            } else {
                number_k_1 = number_k;
                number_k = std::cmp::max(number_k, nums2[end2]);
                k -= end2 - start2 + 1;
                start2 = end2 + 1;
            }
        }

        match len % 2 {
            0 => {
                return (number_k + number_k_1) as f64 / 2.0;
            }
            _ => {
                return number_k as f64;
            }
        }
    }
}

fn main() {
    let nums1 = vec![1,3];
    let nums2 = vec![2,4,5,6];
    println!(
        "find_median_sorted_arrays: {}",
        Solution::find_median_sorted_arrays_2(nums1, nums2)
    );
}

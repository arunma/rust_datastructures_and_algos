use std::collections::{HashMap, BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.is_empty() {return nums}

        let mut freq_map = HashMap::new();
        for num in nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        let mut pq = BinaryHeap::new();
        for (key, val) in freq_map {
            pq.push((val, key))
        }

        let mut result = vec![];
        for _ in 0..k {
            result.push(pq.pop().unwrap().1)
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
        assert_eq!(Solution::top_k_frequent(vec![], 1), vec![]);
    }
}

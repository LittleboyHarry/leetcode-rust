//! # [两数之和](https://leetcode-cn.com/problems/two-sum/) ~ 梦开始的地方
//!
//! 思路参考：([LeetCode B站](https://www.bilibili.com/video/BV1rv411k7VY))
//!
//! 1. 暴力枚举法 —— 遍历所有可能的组合
//! 2. 使用哈希表：数组仅需一次遍历，遍历过的元素要么与当前元素匹配，
//!    要么都存入哈希表（元素为键，下标为值）
//!
//! 方法 1 遍历时仅使用几个临时变量，因此空间复杂度是 $O(1)$.
//! 但因其双层遍历，时间复杂度是 $O(n^2)$，不符合“以空间换时间”的设计思想。
//!
//! 方法 2 只需一层遍历时，时间复杂度是 $O(n)$，本题采用此方法
//!
//! 注：哈希表增删查的时间复杂度都是 $O(1)$，空间复杂度是 $O(n)$.

use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_table: HashMap<i32, usize> = HashMap::new();
        for (index, num) in nums.iter().enumerate() {
            match hash_table.get(&(target - num)) {
                Some(finded_index_ref) => return vec![*finded_index_ref as i32, index as i32],
                None => {
                    hash_table.insert(*num, index);
                }
            }
        }
        return vec![];
    }
}

#[allow(dead_code)]
pub struct Solution;

#[test]
fn case1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

#[test]
fn case2() {
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
}

#[test]
fn case3() {
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}

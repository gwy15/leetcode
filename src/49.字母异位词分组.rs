/*
 * @lc app=leetcode.cn id=49 lang=rust
 *
 * [49] 字母异位词分组
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hmap = HashMap::new();
        for string in strs.into_iter() {
            let mut bytes: Vec<u8> = string.bytes().into_iter().collect();
            bytes.sort_unstable();
            let entry = hmap.entry(bytes).or_insert(vec![]);
            entry.push(string);
        }

        hmap.into_iter().map(|(_k, v)| v).collect()
    }
}
// @lc code=end

/*
 * @lc app=leetcode.cn id=187 lang=rust
 *
 * [187] 重复的DNA序列
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;
impl Solution {
    // ACGT => 0123
    const MASK: i32 = (1 << 20) - 1;
    #[inline(always)]
    fn to_hash(ch: char) -> i32 {
        match ch {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn hash_to_string(mut hash: i32) -> String {
        let mut chars = vec![' '; 10];
        for i in 0..10 {
            let ch = ['A', 'C', 'G', 'T'][(hash & 0b11) as usize];
            hash >>= 2;
            chars[10 - 1 - i] = ch;
        }
        chars.into_iter().collect()
    }

    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        if n < 10 {
            return vec![];
        }
        let mut cnt = HashMap::new();
        // get init hash
        let mut hash = chars[..10]
            .iter()
            .fold(0, |hash, &ch| (hash << 2) | Self::to_hash(ch));
        *cnt.entry(hash).or_insert(0) += 1;

        for i in 10..n {
            // [i, i+10)
            hash = Self::MASK & ((hash << 2) | Self::to_hash(chars[i]));
            *cnt.entry(hash).or_insert(0) += 1;
        }

        let mut ans = vec![];
        for (hash, times) in cnt.into_iter() {
            if times > 1 {
                ans.push(Self::hash_to_string(hash));
            }
        }

        ans
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($s:expr, $ans:tt) => {
            assert_eq!(
                Solution::find_repeated_dna_sequences($s.to_string()),
                vec!$ans
            )
        };
    }
    test!(
        "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT",
        ["AAAAACCCCC", "CCCCCAAAAA"]
    );
}

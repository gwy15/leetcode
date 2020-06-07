/*
 * @lc app=leetcode.cn id=126 lang=rust
 *
 * [126] 单词接龙 II
 */
struct Solution;
// @lc code=start
use std::collections::VecDeque;
#[allow(unused)]
impl Solution {
    fn word_dist(a: &str, b: &str) -> i32 {
        let a = a.as_bytes();
        let b = b.as_bytes();
        let mut dist = 0;
        for i in 0..a.len() {
            if a[i] != b[i] {
                dist += 1;
                if dist >= 2 {
                    return 2;
                }
            }
        }
        dist
    }

    fn word_to_index(word: &str, words: &Vec<String>) -> Option<usize> {
        (0..words.len()).filter(|&i| &words[i] == word).next()
    }

    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        // find index for begin and end
        let begin_i = match Self::word_to_index(&begin_word, &word_list) {
            Some(i) => i,
            None => {
                word_list.push(begin_word);
                word_list.len() - 1
            }
        };
        let end_i = match Self::word_to_index(&end_word, &word_list) {
            Some(i) => i,
            None => return vec![],
        };
        // build graph
        let n = word_list.len();
        let mut next: Vec<Vec<usize>> = vec![vec![]; n];
        for i in 0..n {
            let si = &word_list[i];
            for j in 0..n {
                if i != j && Self::word_dist(si, &word_list[j]) == 1 {
                    next[i].push(j);
                }
            }
        }

        // BFS find dist
        let mut dist = vec![-1; n];
        let mut cur_dist = 0;
        let mut q = VecDeque::new();
        dist[begin_i] = 0;
        q.push_back(begin_i);
        while q.len() > 0 {
            let q_n = q.len();
            for _ in 0..q_n {
                let i = q.pop_front().unwrap();
                if i == end_i {
                    break;
                }
                for &j in &next[i] {
                    if dist[j] == -1 {
                        dist[j] = cur_dist + 1;
                        if j == end_i {
                            break;
                        }
                        q.push_back(j);
                    }
                }
            }
            cur_dist += 1;
        }

        // BFS find paths
        let paths: Vec<Vec<usize>> = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(vec![end_i]);
        let mut ans = Vec::new();
        while q.len() > 0 {
            let q_n = q.len();
            for _ in 0..q_n {
                let path = q.pop_front().unwrap();
                let last_node = *path.last().unwrap();
                for &next_node in &next[last_node] {
                    if dist[next_node] == dist[last_node] - 1 {
                        let mut new_path = path.clone();
                        new_path.push(next_node);
                        if next_node == begin_i {
                            new_path.reverse();
                            ans.push(new_path);
                        } else {
                            // only one node with dist=0
                            q.push_back(new_path);
                        }
                    }
                }
            }
        }

        // ans
        ans.into_iter()
            .map(|path: Vec<usize>| path.into_iter().map(|i| word_list[i].clone()).collect())
            .collect()
    }
}
// @lc code=end
mod utils;
#[test]
fn test_solution() {
    macro_rules! test {
        ($begin:expr, $end:expr, $words:tt, $ans:tt) => {
            let words = vec!$words.into_iter().map(|s| s.to_owned()).collect();
            let ans: Vec<Vec<&str>> = vec2d!$ans;
            assert_eq!(
                Solution::find_ladders($begin.into(), $end.into(), words),
                ans
            );
        }
    };
    test!(
        "hit",
        "cog",
        ["hot", "dot", "dog", "lot", "log", "cog"],
        [
            ["hit", "hot", "dot", "dog", "cog"],
            ["hit", "hot", "lot", "log", "cog"]
        ]
    );
    test!("hit", "cog", ["hot", "dot", "dog", "lot", "log"], []);
}

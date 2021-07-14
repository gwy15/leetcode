/*
 * @lc app=leetcode.cn id=211 lang=rust
 *
 * [211] 添加与搜索单词 - 数据结构设计
 */

// @lc code=start

struct Node {
    children: [Option<Box<Node>>; 26],
    ends_here: bool,
}

impl Node {
    pub fn new() -> Self {
        Self {
            // 什么弱智格式
            children: [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            ends_here: false,
        }
    }

    pub fn add_word(&mut self, word: &[char]) {
        if word.is_empty() {
            self.ends_here = true;
            return;
        }
        let idx = (word[0] as u8 - ('a' as u8)) as usize;

        if self.children[idx].is_none() {
            let mut new_node = Self::new();
            new_node.add_word(&word[1..]);
            self.children[idx] = Some(Box::new(new_node));
        } else {
            self.children[idx].as_mut().unwrap().add_word(&word[1..]);
        }
    }
    pub fn search(&self, word: &[char]) -> bool {
        if word.is_empty() {
            return self.ends_here;
        }
        let ch = word[0];
        if ch == '.' {
            return self
                .children
                .iter()
                .filter_map(|node| node.as_ref())
                .map(|node| node.search(&word[1..]))
                .any(|r| r);
        }
        let idx = (word[0] as u8 - ('a' as u8)) as usize;
        if self.children[idx].is_none() {
            false
        } else {
            self.children[idx].as_ref().unwrap().search(&word[1..])
        }
    }
}

struct WordDictionary {
    root: Node,
}

impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self { root: Node::new() }
    }

    fn add_word(&mut self, word: String) {
        let chars = word.chars().collect::<Vec<_>>();
        self.root.add_word(&chars);
    }

    fn search(&self, word: String) -> bool {
        let chars = word.chars().collect::<Vec<_>>();
        self.root.search(&chars)
    }
}

// @lc code=end
#[test]
fn test_solution() {
    let mut dict = WordDictionary::new();
    dict.add_word("bad".to_string());
    dict.add_word("dad".to_string());
    dict.add_word("mad".to_string());
    assert_eq!(dict.search("pad".to_string()), false);
    assert_eq!(dict.search("bad".to_string()), true);
    assert_eq!(dict.search(".ad".to_string()), true);
    assert_eq!(dict.search("b..".to_string()), true);
}

/*
 * @lc app=leetcode.cn id=381 lang=rust
 *
 * [381] O(1) 时间插入、删除和获取随机元素 - 允许重复
 */

// @lc code=start
use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[allow(unused)]
struct RandomizedCollection {
    items: Vec<i32>,
    indexes: HashMap<i32, HashSet<usize>>,
}

#[allow(unused)]
impl RandomizedCollection {
    /// Initialize your data structure here.
    fn new() -> Self {
        Self {
            items: vec![],
            indexes: HashMap::new(),
        }
    }

    /// Inserts a value to the collection. Returns true if the collection did not already contain the specified element.
    fn insert(&mut self, val: i32) -> bool {
        // insert to items
        let i = self.items.len();
        self.items.push(val);

        match self.indexes.get_mut(&val) {
            None => {
                // not exists
                let mut new_indexes = HashSet::new();
                new_indexes.insert(i);
                self.indexes.insert(val, new_indexes);
                true
            }
            Some(indexes) => {
                // exists
                indexes.insert(i);
                false
            }
        }
    }

    /// Removes a value from the collection. Returns true if the collection contained the specified element.
    fn remove(&mut self, val: i32) -> bool {
        match self.indexes.get_mut(&val) {
            None => {
                // not exists
                false
            }
            Some(indexes) => {
                // exists
                // pop a [index] = val
                let &index = indexes.iter().next().unwrap();
                indexes.remove(&index);
                // remove entry if empty
                if (indexes.len() == 0) {
                    self.indexes.remove(&val);
                }
                // remove [index]=val from items
                // move [end] to [index]
                if index != self.items.len() - 1 {
                    // find an item to swap
                    let last_index = self.items.len() - 1;
                    let &val_swap = self.items.last().unwrap();
                    let indexes = self.indexes.get_mut(&val_swap).unwrap();
                    self.items[index] = val_swap;
                    indexes.remove(&last_index);
                    indexes.insert(index);
                }
                self.items.pop();
                true
            }
        }
    }

    /// Get a random element from the collection.
    fn get_random(&self) -> i32 {
        let mut r = thread_rng();
        let index = r.gen_range(0, self.items.len());
        self.items[index]
    }
}
// @lc code=end

#[test]
fn test_solution() {
    let mut obj = RandomizedCollection::new();
    macro_rules! test {
        (insert, $args:expr, $ans:expr) => {
            assert_eq!(obj.insert($args), $ans)
        };
        (remove, $args:expr, $ans:expr) => {
            assert_eq!(obj.remove($args), $ans)
        };
    }
    test!(insert, 1, true);
    test!(insert, 1, false);
    test!(insert, 2, true);
    obj.get_random();
    test!(remove, 1, true);
    test!(remove, 2, true);
    assert_eq!(obj.get_random(), 1);
}

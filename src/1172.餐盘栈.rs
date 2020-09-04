/*
 * @lc app=leetcode.cn id=1172 lang=rust
 *
 * [1172] 餐盘栈
 */

// @lc code=start
use std::collections::BTreeSet;
#[derive(Debug)]
struct DinnerPlates {
    capacity: usize,
    // [0, right)
    stacks: Vec<Vec<i32>>,
    /// indexes for stack that is insertable (len() < capacity)
    insertable: BTreeSet<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn len(&self) -> usize {
        self.stacks.len()
    }

    fn trim_right(&mut self) {
        while self.stacks.len() > 0 && self.stacks.last().unwrap().len() == 0 {
            let index = self.stacks.len() - 1;
            self.stacks.pop();
            self.insertable.remove(&index);
        }
    }

    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            stacks: Vec::new(),
            insertable: BTreeSet::new(),
        }
    }

    /// push into the leftmost insertable stack.
    /// + algorithm complexity: O(log n)
    fn push(&mut self, val: i32) {
        // find the first insertable
        let index;
        if self.insertable.len() == 0 {
            index = self.len();
            self.stacks.push(Vec::with_capacity(self.capacity));
            self.insertable.insert(index);
        } else {
            index = *self.insertable.iter().next().unwrap();
        }
        // insert to index
        self.stacks[index].push(val);
        // if full, remove from insertable
        if self.stacks[index].len() == self.capacity {
            self.insertable.remove(&index);
        }
    }

    /// pop from the rightmost non-empty stack.
    fn pop(&mut self) -> i32 {
        if self.stacks.len() == 0 {
            return -1;
        }
        let index = self.stacks.len() - 1;
        let size = self.stacks[index].len();
        let val = self.stacks[index].pop().unwrap();
        // if from full to insertable, mark as insertable
        if size == self.capacity {
            self.insertable.insert(index);
        }
        // remove all empty stacks from right
        self.trim_right();
        val
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index = index as usize;
        if index >= self.stacks.len() {
            // not exists
            return -1;
        }
        let size = self.stacks[index].len();
        if size == 0 {
            return -1;
        }
        let val = self.stacks[index].pop().unwrap();
        // from full to insertable
        if size == self.capacity {
            self.insertable.insert(index);
        }
        // remove all empty stacks from right
        self.trim_right();
        val
    }
}

// @lc code=end
#[test]
fn test_solution() {
    let mut p = DinnerPlates::new(2);
    p.push(1);
    p.push(2);
    p.push(3);
    p.push(4);
    p.push(5);
    assert_eq!(p.pop_at_stack(0), 2);
    p.push(20);
    p.push(21);
    assert_eq!(p.pop_at_stack(0), 20);
    assert_eq!(p.pop_at_stack(2), 21);
    assert_eq!(p.pop(), 5);
    assert_eq!(p.pop(), 4);
    assert_eq!(p.pop(), 3);
    assert_eq!(p.pop(), 1);
    assert_eq!(p.pop(), -1);
}

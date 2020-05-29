/*
 * @lc app=leetcode.cn id=732 lang=rust
 *
 * [732] 我的日程安排表 III
 */

// @lc code=start
use std::collections::BTreeMap;

struct MyCalendarThree {
    delta: BTreeMap<i32, i32>,
}

#[allow(unused)]
impl MyCalendarThree {
    fn new() -> Self {
        Self {
            delta: BTreeMap::new(),
        }
    }
    fn book(&mut self, start: i32, end: i32) -> i32 {
        *self.delta.entry(start).or_insert(0) += 1;
        *self.delta.entry(end).or_insert(0) -= 1;
        // validate
        let mut active = 0;
        let mut max_collapse = 0;
        for (_, &delta) in self.delta.iter() {
            active += delta;
            max_collapse = max_collapse.max(active);
        }

        max_collapse
    }
}

// @lc code=end

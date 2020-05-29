/*
 * @lc app=leetcode.cn id=731 lang=rust
 *
 * [731] 我的日程安排表 II
 */

// @lc code=start
use std::collections::BTreeMap;
struct MyCalendarTwo {
    delta: BTreeMap<i32, i32>,
}

#[allow(unused)]
impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            delta: BTreeMap::new(),
        }
    }
    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.delta.entry(start).or_insert(0) += 1;
        *self.delta.entry(end).or_insert(0) -= 1;
        // validate
        let mut active = 0;
        for (_, &delta) in self.delta.iter() {
            active += delta;
            if active >= 3 {
                self.delta.entry(start).and_modify(|n| *n -= 1);
                if self.delta[&start] == 0 {
                    self.delta.remove(&start);
                }
                self.delta.entry(end).and_modify(|n| *n += 1);
                if self.delta[&end] == 0 {
                    self.delta.remove(&end);
                }
                return false;
            }
        }

        true
    }
}

// @lc code=end

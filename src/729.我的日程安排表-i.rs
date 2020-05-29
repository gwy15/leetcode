/*
 * @lc app=leetcode.cn id=729 lang=rust
 *
 * [729] 我的日程安排表 I
 */

// @lc code=start
use std::collections::BTreeMap;

#[allow(unused)]
struct CalendarItem {
    start: i32,
    end: i32,
}

struct MyCalendar {
    items: BTreeMap<i32, CalendarItem>,
}

#[allow(unused)]
impl MyCalendar {
    fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        use std::ops::Bound::{Included, Unbounded};
        // validate next item
        if let Some(next) = self.items.range((Included(start), Unbounded)).next() {
            let (&next_start, _) = next;
            if end > next_start {
                return false;
            }
        }
        // validate prev item
        if let Some(prev) = self.items.range((Unbounded, Included(start))).last() {
            let (&_, prev_item) = prev;
            if start < prev_item.end {
                return false;
            }
        }
        // insert
        self.items.insert(start, CalendarItem { start, end });

        true
    }
}

// @lc code=end

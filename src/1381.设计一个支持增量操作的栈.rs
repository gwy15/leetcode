/*
 * @lc app=leetcode.cn id=1381 lang=rust
 *
 * [1381] 设计一个支持增量操作的栈
 */

// @lc code=start
struct CustomStack {
    deltas: Vec<i32>,
    end: i32,
    max_size: usize,
}

impl CustomStack {
    fn new(maxSize: i32) -> Self {
        Self {
            deltas: Vec::new(),
            max_size: maxSize as usize,
            end: 0,
        }
    }

    fn len(&self) -> usize {
        self.deltas.len()
    }

    fn push(&mut self, x: i32) {
        if self.len() < self.max_size {
            // end -> x
            let delta = x - self.end;
            self.deltas.push(delta);
            self.end = x;
        }
    }

    fn pop(&mut self) -> i32 {
        if self.len() == 0 {
            return -1;
        }
        let ans = self.end;
        let delta = self.deltas.pop().unwrap();
        let last = self.end - delta;
        self.end = last;
        ans
    }

    fn increment(&mut self, k: i32, val: i32) {
        if self.len() == 0 {
            return;
        }
        let k = k as usize;
        // add delta for element 0
        self.deltas[0] += val;
        if k >= self.len() {
            // all elements += val
            self.end += val;
        } else {
            self.deltas[k] -= val;
        }
    }
}

// @lc code=end
#[test]
fn test_solution() {
    let mut s = CustomStack::new(3); // 栈是空的 []
    s.push(1); // 栈变为 [1]
    s.push(2); // 栈变为 [1, 2]
    assert_eq!(s.pop(), 2); // 返回 2 --> 返回栈顶值 2，栈变为 [1]
    s.push(2); // 栈变为 [1, 2]
    s.push(3); // 栈变为 [1, 2, 3]
    s.push(4); // 栈仍然是 [1, 2, 3]，不能添加其他元素使栈大小变为 4
    s.increment(5, 100); // 栈变为 [101, 102, 103]
    s.increment(2, 100); // 栈变为 [201, 202, 103]
    assert_eq!(s.pop(), 103); // 返回 103 --> 返回栈顶值 103，栈变为 [201, 202]
    assert_eq!(s.pop(), 202); // 返回 202 --> 返回栈顶值 202，栈变为 [201]
    assert_eq!(s.pop(), 201); // 返回 201 --> 返回栈顶值 201，栈变为 []
    assert_eq!(s.pop(), -1); // 返回 -1 --> 栈为空，返回 -1
}

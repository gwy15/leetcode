/*
 * @lc app=leetcode.cn id=155 lang=rust
 *
 * [155] 最小栈
 */
struct Solution;
// @lc code=start
#[derive(Debug)]
struct MinElement {
    pub element: i32,
    pub count: usize,
}

#[derive(Debug)]
struct MinStack {
    elements: Vec<i32>,
    min_elements: Vec<MinElement>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            elements: Vec::new(),
            min_elements: Vec::new(),
        }
    }
    fn push(&mut self, x: i32) {
        self.elements.push(x);
        if let Some(min_element) = self.min_elements.last_mut() {
            if min_element.element <= x {
                min_element.count += 1;
                return;
            }
        }
        self.min_elements.push(MinElement {
            element: x,
            count: 1,
        });
    }

    fn pop(&mut self) {
        self.elements.pop();
        let last = self.min_elements.last_mut().unwrap();
        if last.count > 1 {
            last.count -= 1;
        } else {
            self.min_elements.pop();
        }
    }
    fn top(&self) -> i32 {
        *self.elements.last().unwrap()
    }
    fn get_min(&self) -> i32 {
        self.min_elements.last().unwrap().element
    }
}

// @lc code=end
#[test]
fn test_solution() {
    let mut s = MinStack::new();
    s.push(-2);
    s.push(0);
    s.push(-3);
    assert_eq!(s.get_min(), -3);
    s.pop();
    assert_eq!(s.top(), 0);
    assert_eq!(s.get_min(), -2);
}

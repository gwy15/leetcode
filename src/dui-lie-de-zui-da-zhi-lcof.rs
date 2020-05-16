use std::collections::VecDeque;

#[derive(Debug)]
struct _MaxQueue<T> {
    /// elements
    q: VecDeque<T>,
    /// max elements, a decreasing queue
    max: VecDeque<(T, u32)>,
}

#[allow(unused)]
impl<T> _MaxQueue<T>
where
    T: std::cmp::Ord + std::clone::Clone,
{
    /// Create a new max queue
    pub fn new() -> Self {
        Self {
            q: VecDeque::new(),
            max: VecDeque::new(),
        }
    }
    /// push element to queue back
    pub fn push_back(&mut self, item: T) {
        // maintain max elements
        use std::cmp::Ordering::*;
        let mut count = 1;
        loop {
            if self.max.len() == 0 {
                self.max.push_back((item.clone(), count));
                break;
            }
            let (last_element, last_times) = self.max.back().unwrap();
            match last_element.cmp(&item) {
                Greater => {
                    self.max.push_back((item.clone(), count));
                    break;
                }
                Equal => {
                    // times += count
                    self.max.back_mut().unwrap().1 += count;
                    break;
                }
                Less => {
                    count += last_times;
                    self.max.pop_back().unwrap();
                    continue;
                }
            }
        }

        // push to elements
        self.q.push_back(item);
    }
    /// pop first (front) element from queue
    pub fn pop_front(&mut self) -> Option<T> {
        match self.q.pop_front() {
            None => None,
            Some(item) => {
                let max_element = self.max.front_mut().unwrap();
                if max_element.1 == 1 {
                    self.max.pop_front().unwrap();
                } else {
                    max_element.1 -= 1;
                }

                Some(item)
            }
        }
    }
    /// Length of the queue
    pub fn len(&self) -> usize {
        self.q.len()
    }
    /// Get max element from queue
    pub fn max(&self) -> Option<&T> {
        match self.max.front() {
            None => None,
            Some((max_element, _)) => Some(max_element),
        }
    }
}

struct MaxQueue {
    q: _MaxQueue<i32>,
}

impl MaxQueue {
    fn new() -> Self {
        Self {
            q: _MaxQueue::new(),
        }
    }
    fn max_value(&self) -> i32 {
        *self.q.max().unwrap_or(&-1)
    }
    fn push_back(&mut self, value: i32) {
        self.q.push_back(value)
    }
    fn pop_front(&mut self) -> i32 {
        self.q.pop_front().unwrap_or(-1)
    }
}

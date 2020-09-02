/*
 * @lc app=leetcode.cn id=1146 lang=rust
 *
 * [1146] 快照数组
 */

// @lc code=start
struct SnapshotArray {
    nums: Vec<Vec<(i32, i32)>>,
    t: i32,
    // dirty: bool,
}
#[allow(unused)]
impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            nums: vec![vec![]; length as usize],
            t: 0,
            // dirty: false,
        }
    }
    fn set(&mut self, index: i32, val: i32) {
        let arr = &mut self.nums[index as usize];
        if let Some(&(t, old_val)) = arr.last() {
            if val == old_val {
                return;
            }
            // val != old_val
            if t == self.t {
                arr.last_mut().unwrap().1 = val;
                // self.dirty = true;
                return;
            }
        }
        arr.push((self.t, val));
        // self.dirty = true;
    }
    fn snap(&mut self) -> i32 {
        let t = self.t;
        // if self.dirty {
        self.t += 1;
        // self.dirty = false;
        // }
        t
    }
    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let arr = &self.nums[index as usize];
        let n = arr.len();
        if n == 0 {
            return 0;
        }
        let (mut left, mut right) = (0, n);
        // binary search
        while left < right {
            let mid = (left + right) / 2;
            let t = arr[mid].0;
            if t == snap_id {
                // find snap_id
                return arr[mid].1;
            } else if t < snap_id {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        // left == right
        // eprintln!("  => search {} result in {:?}: left={}", snap_id, arr, left);
        if left == 0 {
            return 0;
        }
        // insert at left, so t is at left-1
        arr[left - 1].1
    }
}

// @lc code=end
#[test]
fn test_solution() {
    let mut s = SnapshotArray::new(3);
    s.set(0, 5);
    let t = s.snap();
    s.set(0, 6);
    assert_eq!(s.get(t, 0), 5);

    let mut s = SnapshotArray::new(1);
    s.set(0, 4);
    s.set(0, 16);
    s.set(0, 13);
    assert_eq!(0, s.snap());
    assert_eq!(s.get(0, 0), 13);

    let mut s = SnapshotArray::new(4);
    assert_eq!(s.snap(), 0);
    assert_eq!(s.snap(), 1);
    assert_eq!(s.get(3, 1), 0);
    s.set(2, 4);
    assert_eq!(s.snap(), 2);
    s.set(1, 4);

    let mut s = SnapshotArray::new(1);
    s.set(0, 15);
    s.snap();
    s.snap();
    s.snap();
    s.get(0, 2);
    s.snap();
    s.snap();
    s.get(0, 0);

    let mut s = SnapshotArray::new(2);
    s.snap();
    s.get(1, 0);
    s.get(0, 0);
    s.set(1, 8);
    s.get(1, 0);
    s.set(0, 20);
    s.get(0, 0);
    s.set(0, 7);
}

/*
 * @lc app=leetcode.cn id=278 lang=rust
 *
 * [278] 第一个错误的版本
 */
struct Solution {
    n: i32,
    bad: i32,
}

// @lc code=start
// The API isBadVersion is defined for you.
// to call it use self.isBadVersion(versions)

impl Solution {
    // fn new(n: i32, bad: i32) -> Self {
    //     Self { n, bad }
    // }
    // fn isBadVersion(&self, versions: i32) -> bool {
    //     versions >= self.bad
    // }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        // 可能位置： [1, n]
        let (mut left, mut right) = (1, n);
        while left < right {
            let mid = left + (right - left) / 2;
            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        return left;
    }
}
// @lc code=end
#[test]
fn test_solution() {
    macro_rules! test {
        ($n:expr, $ans:expr) => {{
            let s = Solution::new($n, $ans);
            assert_eq!(s.first_bad_version($n), $ans)
        }};
    }
    test!(5, 4);
    test!(2126753390, 1702766719);
}

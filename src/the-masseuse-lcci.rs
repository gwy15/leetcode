struct Solution;

impl Solution {
    pub fn massage(nums: Vec<i32>) -> i32 {
        let (mut select, mut pass) = (0, 0);
        for num in nums {
            let _select = pass + num;
            let _pass = i32::max(select, pass);
            select = _select;
            pass = _pass;
        }
        select.max(pass)
    }
}

#[test]
fn test_solution() {
    assert_eq!(Solution::massage(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::massage(vec![2, 1, 4, 5, 3, 1, 1, 3]), 12);
}

struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let (mut i, n) = (0, pushed.len());
        for item in pushed {
            stack.push(item);
            while i < n && Some(&popped[i]) == stack.last() {
                stack.pop();
                i += 1;
            }
        }
        i == n
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($a:tt, $b:tt, $ans:expr) => {
            assert_eq!(
                Solution::validate_stack_sequences(vec!$a, vec!$b),
                $ans
            )
        };
    }
    test!([1, 2, 3, 4, 5], [4, 5, 3, 2, 1], true);
    test!([1, 2, 3, 4, 5], [4, 3, 5, 1, 2], false);
}

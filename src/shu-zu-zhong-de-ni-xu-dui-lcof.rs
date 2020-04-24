struct Solution;

impl Solution {
    fn _reverse_pairs(nums: &mut [i32]) -> usize {
        let n = nums.len();
        match n {
            0 | 1 => {
                return 0;
            }
            2 => {
                if nums[0] > nums[1] {
                    nums.swap(0, 1);
                    return 1;
                } else {
                    return 0;
                }
            }
            _ => {}
        }
        let mid = n / 2;
        let mut ans =
            Solution::_reverse_pairs(&mut nums[..mid]) + Solution::_reverse_pairs(&mut nums[mid..]);
        // merge
        let (mut l, mut r) = (0, mid);
        let mut tmp = Vec::new();
        tmp.reserve(n);
        while l < mid && r < n {
            if nums[l] <= nums[r] {
                tmp.push(nums[l]);
                l += 1;
                ans += r - mid;
            } else {
                tmp.push(nums[r]);
                r += 1;
            }
        }
        // 处理剩余
        for i in l..mid {
            tmp.push(nums[i]);
            ans += r - mid;
        }
        for i in r..n {
            tmp.push(nums[i]);
        }
        nums.copy_from_slice(&tmp[..]);
        ans
    }

    pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
        Solution::_reverse_pairs(&mut nums) as i32
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($v:tt, $a:expr) => {
            assert_eq!(
                Solution::reverse_pairs(vec!$v),
                $a
            );
        }
    };
    test!([7, 5, 6, 4], 5);
    test!([5, 1, 2, 3, 4, 6], 4);
    test!([], 0);
    test!([1], 0);
    test!([1, 2, 3], 0);
    test!([1, 3, 2, 4], 1);
}

struct Solution;

use std::cmp::Ordering::*;
const MOD: i32 = 1_000_000_007;

impl Solution {
    /// 以 base 为被除数的总和
    fn sum_of_target(nums: &[i32], base: i32) -> i32 {
        let n = nums.len();
        let (mut i, mut j) = (0, 0);
        let mut ans = 0;
        let mut times = 1;
        while j < n {
            // 找到第一个大于等于 ai * k 的 => j
            let ret = nums.binary_search_by(|aj| match aj.cmp(&(times * base)) {
                Less => Less,
                // 只要大于等于就行
                Equal | Greater => Greater,
            });
            j = ret.unwrap_err();
            // i..j 要加上去 (j-i) * times
            ans += (j - i) as i32 * (times - 1);
            ans %= MOD;

            i = j;
            // 设置 times 为 a[j] 下一个
            if j < n {
                times = (nums[j] / base) + 1;
            } else {
                times += 1;
            }
        }
        // println!("base = {}, sum = {}", base, ans);
        ans
    }

    pub fn sum_of_floored_pairs(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        match n {
            0 => return 0,
            1 => return 1,
            _ => {}
        }
        // n log n
        nums.sort_unstable();
        let mut ans = 0;

        // 这里可以优化，同一个 base 只做一次
        let mut idx = 0;
        while idx < n {
            // 只需要考虑 a[j] >= a[i] 的 a[j] / a[i]，小于 a[i] 的不会对结果有影响
            let base = nums[idx];

            let base_ret = Self::sum_of_target(&nums, base);

            while idx < n && nums[idx] == base {
                ans += base_ret;
                ans %= MOD;
                idx += 1;
            }
        }
        ans
    }
}

#[test]
fn test_solution() {
    macro_rules! test {
        ($args:tt, $ans:expr) => {
            assert_eq!(
                Solution::sum_of_floored_pairs(vec!$args),
                $ans
            );
        };
    }
    test!([2, 5, 9], 10);
    test!([7, 7, 7, 7, 7, 7, 7], 49);
    test!(
        [
            10, 78, 25, 66, 29, 33, 55, 13, 81, 26, 70, 71, 13, 26, 9, 25, 91, 93, 91, 44, 27, 96,
            38, 22, 98, 13, 10, 69, 72, 25, 67, 15, 97, 94, 27, 58, 64, 57, 13, 75, 88, 42, 56, 80,
            7, 13, 19, 65, 13, 56
        ],
        3384
    );
    test!(
        [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 10000
        ],
        806401
    );
}

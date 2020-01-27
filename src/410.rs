struct Solution {}

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        // Self::split_array_dp(nums, m)
        Self::split_array_partition(&nums, m)
    }
    pub fn split_array_dp(nums: Vec<i32>, m: i32) -> i32 {
        let n = nums.len();

        // cache for _sum[i] = nums[0] + .. + nums[i]
        let mut _sum = vec![0; n];
        _sum[0] = nums[0];
        for i in 1..n {
            _sum[i] = _sum[i - 1] + nums[i];
        }
        // sum_part(i, j) = nums[i] + nums[i+1] + .. + nums[j]
        // O(1)
        let sum_part = |i, j| {
            if i == 0 {
                _sum[j]
            } else {
                _sum[j] - _sum[i - 1]
            }
        };
        // dp: dp[i][j] = first i elements, j groups
        // formula:
        // dp[i][j] = max(
        //     dp[k][j-1] + sum(nums[k+1 .. i][j])
        //     for k in ...
        // )
        // notice only j is used, simplify to dp[i]
        // here, dp[i] = nums[0 .. i].sum()
        let max: i32 = nums.iter().sum();

        let mut last_dp = vec![max; n];
        // j: num of groups
        for j in 1..m + 1 {
            let mut dp = vec![max; n]; // i+1 >= j
            let start_i = (j - 1) as usize;
            for i in start_i..n {
                // dp[i] = last_dp[k] + sum(k+1, i)
                // k < i, k+1 >= j-1, k in range [j-2, i)
                dp[i] = (j - 2..i as i32)
                    .map(|k: i32| {
                        if k < 0 {
                            sum_part((k + 1) as usize, i as usize)
                        } else {
                            last_dp[k as usize].max(sum_part((k + 1) as usize, i as usize))
                        }
                    })
                    .min()
                    .unwrap();
                // println!("calc dp[{}][{}] = {}", i, j, dp[i]);
            }
            // println!("j = {}, dp[i][{}] = {:?}", j, j, dp);
            last_dp = dp;
        }
        // return
        last_dp[n - 1]
    }

    pub fn split_array_partition(nums: &[i32], m: i32) -> i32 {
        // println!("== splitting arr {:?}, m {}", nums, m);
        let mut low = *nums.iter().max().unwrap();
        if (low == std::i32::MAX) {
            return low;
        }
        let mut high = nums.iter().sum();
        let mut ans = high; // ans must ok
                            // [low, high]
        while low < high {
            // println!("Searching range [low={}, high={}] m={}", low, high, m);
            let mid = low + (high - low) / 2; // fall back to low
            let mid_ok = Self::is_array_splitable_with_max_partial_sum_of_sections(&nums, m, mid);
            // println!("testing mid = {}, ok = {}", mid, mid_ok);
            if mid_ok {
                ans = ans.min(mid);
                // mid ok, try better
                high = mid - 1;
            } else {
                // failed
                low = mid + 1;
            }
            // println!("current low = {}, high = {}", low, high);
        }
        if (Self::is_array_splitable_with_max_partial_sum_of_sections(&nums, m, low)) {
            ans = low;
        }
        ans
    }

    fn is_array_splitable_with_max_partial_sum_of_sections(
        nums: &[i32],
        sections: i32,
        max_sum: i32,
    ) -> bool {
        //assert!(max_sum >= nums.iter().max());
        let mut section_cnt = 0;
        let mut partial_sum: i32 = 0;
        for num in nums {
            // partial_sum < num
            partial_sum += num;
            if partial_sum > max_sum {
                // allow equal, num could be zero
                section_cnt += 1;
                if section_cnt > sections {
                    return false;
                }
                // reset
                partial_sum = *num;
            }
        }
        section_cnt += 1;
        section_cnt <= sections
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let arr = vec![7, 2, 5, 10, 8];
        assert_eq!(Solution::split_array(arr, 2), 18);
        let arr = vec![10, 11];
        assert_eq!(Solution::split_array(arr, 2), 11);

        let arr = vec![1, 2, 3];
        assert_eq!(Solution::split_array(arr, 1), 6);
        let arr = vec![1, 2, 3];
        assert_eq!(Solution::split_array(arr, 2), 3);
        let arr = vec![1, 2, 3];
        assert_eq!(Solution::split_array(arr, 3), 3);

        let arr = vec![1, 2147483646];
        assert_eq!(Solution::split_array(arr, 1), 2147483647);

        let arr = vec![1, 2147483647];
        assert_eq!(Solution::split_array(arr, 2), 2147483647);
    }
}

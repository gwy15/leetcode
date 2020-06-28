#
# @lc app=leetcode.cn id=209 lang=python3
#
# [209] 长度最小的子数组
#
from typing import List
# @lc code=start


class Solution:
    def minSubArrayLen(self, s: int, nums: List[int]) -> int:
        n = len(nums)
        # [left, right)
        left, right = 0, 0
        cur_sum = 0
        ans = n + 1
        while left < n:
            if cur_sum < s:
                if right == n:
                    break
                cur_sum += nums[right]
                right += 1
            else:  # cur_sum >= s
                ans = min(ans, right - left)
                cur_sum -= nums[left]
                left += 1

        if ans == n+1:
            return 0
        else:
            return ans


# @lc code=end
if __name__ == "__main__":
    s = Solution()
    f = s.minSubArrayLen
    assert f(7, [2, 3, 1, 2, 4, 3]) == 2
    assert f(200, [1, 1, 1]) == 0
    assert f(3, [1, 1, 1]) == 3
    assert f(0, []) == 0

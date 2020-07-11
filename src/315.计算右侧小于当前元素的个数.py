#
# @lc app=leetcode.cn id=315 lang=python3
#
# [315] 计算右侧小于当前元素的个数
#
from typing import List
# @lc code=start
from sortedcontainers import SortedList


class Solution:
    def countSmaller(self, nums: List[int]) -> List[int]:
        n = len(nums)
        arr = SortedList()
        ans = [0] * n
        for i in range(n-1, -1, -1):
            #
            count = arr.bisect_left(nums[i])
            ans[i] = count

            arr.add(nums[i])
        return ans


# @lc code=end
if __name__ == "__main__":
    assert Solution().countSmaller([5, 2, 6, 1]) == [2, 1, 1, 0]
    assert Solution().countSmaller([1]) == [0]
    assert Solution().countSmaller([1, 1, 1, 1, 1, 1]) == [0, 0, 0, 0, 0, 0]

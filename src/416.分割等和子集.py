#
# @lc app=leetcode.cn id=416 lang=python3
#
# [416] 分割等和子集
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        total = sum(nums)
        if total % 2 == 1:
            return False
        target = total // 2
        can_sum = [False for _ in range(target + 1)]
        can_sum[0] = True
        for n in nums:
            if target - n >= 0 and can_sum[target - n]:
                return True
            for i in range(target-n, -1, -1):
                if can_sum[i]:
                    can_sum[i + n] = True

        return False


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().canPartition(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 5, 11, 5], True)
    test([1, 2, 3, 5], False)
    test([1, 2, 5], False)

#
# @lc app=leetcode.cn id=1546 lang=python3
#
# [1546] 和为目标值且不重叠的非空子数组的最大数目
#
from prelude import *
# @lc code=start


class Solution:
    def maxNonOverlapping(self, nums: List[int], target: int) -> int:
        n = len(nums)
        count = 0

        i = 0
        while i < n:
            # 从 i 开始计算前缀和
            prefixes = {0}
            prefix = 0
            while i < n:
                prefix += nums[i]
                if prefix - target in prefixes:
                    count += 1
                    break
                else:
                    prefixes.add(prefix)
                    i += 1
            # 找到了一个子数组
            i += 1

        return count


# @lc code=end
if __name__ == '__main__':
    def test(nums, target, expected):
        calc = Solution().maxNonOverlapping(nums, target)
        if calc != expected:
            print(f'case failed: `{nums, target}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(nums=[1, 1, 1, 1, 1], target=2, expected=2)
    test(nums=[-1, 3, 5, 1, 4, 2, -9], target=6, expected=2)
    test(nums=[-2, 6, 6, 3, 5, 4, 1, 2, 8], target=10, expected=3)
    test(nums=[0, 0, 0], target=0, expected=3)

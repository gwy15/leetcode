#
# @lc app=leetcode.cn id=11 lang=python3
#
# [11] 盛最多水的容器
#
from prelude import *
# @lc code=start


class Solution:
    def maxArea(self, height: List[int]) -> int:
        n = len(height)
        best = 0
        i, j = 0, n-1
        while i < j:
            best = max(
                best,
                min(height[i], height[j]) * (j - i)
            )
            if height[i] < height[j]:
                i += 1
            else:
                j -= 1
        return best


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().maxArea(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 8, 6, 2, 5, 4, 8, 3, 7], 49)
    test([1, 1], 1)
    test([4, 3, 2, 1, 4], 16)
    test([1, 2, 1], 2)

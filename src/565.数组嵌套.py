#
# @lc app=leetcode.cn id=565 lang=python3
#
# [565] 数组嵌套
#
from typing import List

# @lc code=start


class DSU:
    def __init__(self, n):
        self.f = [i for i in range(n)]
        self.size = [1] * n

    def find(self, x):
        if self.f[x] != x:
            self.f[x] = self.find(self.f[x])
        return self.f[x]

    def union(self, x, y):
        x = self.find(x)
        y = self.find(y)
        if x == y:
            return self.size[x]
        if self.size[x] <= self.size[y]:
            x, y = y, x
        # y -> x
        self.f[y] = x
        self.size[x] += self.size[y]
        return self.size[x]


class SolutionDSU:
    def arrayNesting(self, nums: List[int]) -> int:
        n = len(nums)
        ans = 0
        dsu = DSU(n)
        for x in nums:
            y = nums[x]
            size = dsu.union(x, y)
            ans = max(ans, size)
        return ans


class Solution:
    def arrayNesting(self, nums: List[int]) -> int:
        ans = 0
        for i in range(len(nums)):
            cnt = 0
            while nums[i] != -1:
                j = i
                cnt += 1
                i = nums[i]
                nums[j] = -1
            ans = max(ans, cnt)
        return ans


# @lc code=end
if __name__ == "__main__":
    s = Solution()
    assert s.arrayNesting([5, 4, 0, 3, 1, 6, 2]) == 4
    assert s.arrayNesting([0]) == 1

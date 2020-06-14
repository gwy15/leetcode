#
# @lc app=leetcode.cn id=1300 lang=python3
#
# [1300] 转变数组后最接近目标值的数组和
#
import unittest
from typing import List
# @lc code=start

import math


class Solution:
    def findBestValue(self, arr: List[int], target: int) -> int:
        n = len(arr)

        # 超过范围，直接全体 crop
        if n * min(arr) >= target:
            v = target / n
            if abs(n * math.floor(v) - target) <= abs(n * math.ceil(v) - target):
                return math.floor(v)
            else:
                return math.ceil(v)
        # 排序
        arr.sort()
        # 前缀和
        pre = arr[:]
        for i in range(1, n):
            pre[i] += pre[i-1]

        def sum(i, value):
            return pre[i] + value * (n - i - 1)

        # 达不到范围，返回最大
        if pre[-1] < target:
            return max(arr)

        # 二分查找
        left, right = 0, n  # [left, right)
        while left + 1 < right:
            mid = (left + right) // 2
            # 将 value 设为 arr[mid]
            sum_ = sum(mid, arr[mid])
            if sum_ == target:
                return arr[mid]
            elif sum_ < target:
                left = mid
            else:
                right = mid
        # 现在， arr[left] 为 value 偏小， arr[left+1] 偏大。需要找到中点
        # 有 pre[left] + value(n - left - 1) ~= target
        v = (target - pre[left]) / (n - left - 1)
        # 尝试 floor(v) 和 ceil(v)
        v_floor, v_ceil = math.floor(v), math.ceil(v)
        sum_floor = sum(left, v_floor)
        sum_ceil = sum(left, v_ceil)
        if abs(sum_floor - target) <= abs(sum_ceil - target):
            return v_floor
        else:
            return v_ceil


# @lc code=end
class Test(unittest.TestCase):
    def test(self):
        s = Solution()

        def test(arr, target, ans):
            self.assertEqual(s.findBestValue(arr, target), ans)

        test([4, 9, 3], 10, 3)
        test([2, 3, 5], 10, 5)
        test([60864, 25176, 27249, 21296, 20204], 56803, 11361)
        test([1, 2, 3], 100, 3)


if __name__ == "__main__":
    unittest.main()

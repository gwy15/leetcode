#
# @lc app=leetcode.cn id=977 lang=python3
#
# [977] 有序数组的平方
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def sortedSquares(self, A: List[int]) -> List[int]:
        n = len(A)
        ans = [0] * n
        i, j, k = 0, n, n
        while i < j:
            k -= 1
            if abs(A[i]) >= abs(A[j-1]):
                ans[k] = A[i] ** 2
                i += 1
            else:
                j -= 1
                ans[k] = A[j] ** 2

        return ans
# @lc code=end

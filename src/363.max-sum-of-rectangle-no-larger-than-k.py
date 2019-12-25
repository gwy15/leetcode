#
# @lc app=leetcode id=363 lang=python3
#
# [363] Max Sum of Rectangle No Larger Than K
#
from typing import List
# @lc code=start
import numpy as np


class Solution:
    def maxSumSubmatrix(self, matrix: List[List[int]], k: int) -> int:
        sizeX, sizeY = len(matrix), len(matrix[0])
        dp = np.zeros((sizeX, sizeY), dtype=int)
        sumCol = np.zeros((sizeY, ), dtype=int)
        for i in range(sizeX):
            col = matrix[i]
            sumCol[0] = col[0]
            for j in range(1, sizeY):
                sumCol[j] = sumCol[j-1] + col[j]
            # push to dp
            dp[i, :] = dp[i-1, :] + sumCol
        # dp finished
        

# @lc code=end

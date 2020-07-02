#
# @lc app=leetcode.cn id=378 lang=python3
#
# [378] 有序矩阵中第K小的元素
#
from typing import List
# @lc code=start


class Solution:
    def count_le(self, matrix: List[List[int]], mid: int) -> int:
        n = len(matrix)
        count = 0
        i = n
        for row in range(n):
            while i > 0 and matrix[row][i-1] > mid:
                i -= 1
            count += i

        return count

    def kthSmallest(self, matrix: List[List[int]], k: int) -> int:
        n = len(matrix)
        left = matrix[0][0]
        right = matrix[n-1][n-1]
        while left < right:
            mid = (left + right) // 2
            count = self.count_le(matrix, mid)
            if count >= k:
                right = mid
            else:
                left = mid + 1
        return left


# @lc code=end
if __name__ == "__main__":
    f = Solution().kthSmallest
    arr = [
        [1,  5,  9],
        [10, 11, 13],
        [12, 13, 15]
    ]
    assert f(arr, 8) == 13
    assert f(arr, 9) == 15
    assert f(arr, 1) == 1

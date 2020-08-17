#
# @lc app=leetcode.cn id=48 lang=python3
#
# [48] 旋转图像
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def rotate(self, matrix: List[List[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """
        # rotate block
        #   j  ..(n+1)/2
        # i 0 1   2 | 3 4            0 1 | 2 3
        #   0 1   2 | 3 4            0 1 | 2 3
        #   ----+---|          or    ----+----
        #   0 1 | 2 | 3 4            0 1 | 2 3
        #       +---+----            0 1 | 2 3
        #   0 1 | 2   3 4
        #   0 1 | 2   3 4
        # ..n/2
        n = len(matrix)
        for i in range(n // 2):
            for j in range((n+1) // 2):
                ai, aj = i, j
                bi, bj = j, n-1-i
                ci, cj = n-1-i, n-1-j
                di, dj = n-1-j, i
                #
                a = matrix[ai][aj]
                matrix[ai][aj] = matrix[di][dj]
                matrix[di][dj] = matrix[ci][cj]
                matrix[ci][cj] = matrix[bi][bj]
                matrix[bi][bj] = a


# @lc code=end
if __name__ == "__main__":
    def test(a, b):
        Solution().rotate(a)
        # for i in a:
        #     print(i)
        assert a == b
    test([
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ], [
        [7, 4, 1],
        [8, 5, 2],
        [9, 6, 3]
    ])
    test(
        [
            [5, 1, 9, 11],
            [2, 4, 8, 10],
            [13, 3, 6, 7],
            [15, 14, 12, 16]
        ], [
            [15, 13, 2, 5],
            [14, 3, 4, 1],
            [12, 6, 8, 9],
            [16, 7, 10, 11]
        ]
    )

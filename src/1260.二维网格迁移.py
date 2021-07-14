#
# @lc app=leetcode.cn id=1260 lang=python3
#
# [1260] 二维网格迁移
#
from prelude import *
# @lc code=start


class Solution:
    def shiftGrid(self, grid: List[List[int]], k: int) -> List[List[int]]:
        if k == 0 or len(grid) == 0:
            return grid
        # 没说空间要 O(1)
        m, n = len(grid), len(grid[0])
        if k % (m * n) == 0:
            return grid

        # i, j => pos => pos + k => i', j'

        def ij2pos(i, j):
            return i * n + j

        def pos2ij(pos):
            return pos // n, pos % n

        def new_value(i, j):
            pos = ij2pos(i, j)
            i, j = pos2ij((pos - k + m * n) % (m * n))
            return grid[i][j]

        new = [
            [
                new_value(i, j)
                for j in range(n)
            ]
            for i in range(m)
        ]
        return new


# @lc code=end
if __name__ == '__main__':
    def test(grid, k, expected):
        calc = Solution().shiftGrid(grid, k)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        grid=[[1, 2, 3], [4, 5, 6], [7, 8, 9]], k=1,
        expected=[[9, 1, 2], [3, 4, 5], [6, 7, 8]]
    )
    test(
        grid=[[3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10], [12, 0, 21, 13]], k=4,
        expected=[[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]]
    )
    test(
        grid=[[1, 2, 3], [4, 5, 6], [7, 8, 9]], k=9,
        expected=[[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    )

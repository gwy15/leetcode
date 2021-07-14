#
# @lc app=leetcode.cn id=1034 lang=python3
#
# [1034] 边框着色
#
from prelude import *
# @lc code=start


class Solution:

    def valid(self, grid, x, y):
        return 0 <= x < self.m and 0 <= y < self.n and grid[x][y] == self.target_color

    def dfs(self, grid, x, y):
        """(x, y) 一定是合法的连通分量，但不一定是没访问过的
        """
        if (x, y) in self.visited:
            return
        self.visited.add((x, y))

        valid_neighbors = 0
        if self.valid(grid, x + 1, y):
            self.dfs(grid, x + 1, y)
            valid_neighbors += 1
        if self.valid(grid, x - 1, y):
            self.dfs(grid, x - 1, y)
            valid_neighbors += 1

        if self.valid(grid, x, y+1):
            self.dfs(grid, x, y+1)
            valid_neighbors += 1
        if self.valid(grid, x, y-1):
            self.dfs(grid, x, y-1)
            valid_neighbors += 1

        if valid_neighbors < 4:
            self.ans[x][y] = self.border_color

    def colorBorder(self, grid: List[List[int]], r0: int, c0: int, color: int) -> List[List[int]]:
        if len(grid) == 0:
            return grid

        self.m, self.n = len(grid), len(grid[0])
        self.visited = set()
        self.ans = [r[:] for r in grid]
        self.target_color = grid[r0][c0]
        self.border_color = color

        self.dfs(grid, r0, c0)
        return self.ans


# @lc code=end
if __name__ == '__main__':
    def test(grid, r0, c0, color, expected):
        calc = Solution().colorBorder(grid, r0, c0, color)
        if calc != expected:
            print(f'case failed: `{grid, r0, c0, color}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        grid=[[1, 1], [1, 2]], r0=0, c0=0, color=3,
        expected=[[3, 3], [3, 2]]
    )
    test(
        grid=[[1, 2, 2], [2, 3, 2]], r0=0, c0=1, color=3,
        expected=[[1, 3, 3], [2, 3, 3]]
    )
    test(
        grid=[[1, 1, 1], [1, 1, 1], [1, 1, 1]], r0=1, c0=1, color=2,
        expected=[[2, 2, 2], [2, 1, 2], [2, 2, 2]]
    )

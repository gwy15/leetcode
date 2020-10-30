#
# @lc app=leetcode.cn id=463 lang=python3
#
# [463] 岛屿的周长
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def islandPerimeter(self, grid: List[List[int]]) -> int:
        m = len(grid)
        if m == 0:
            return 0
        n = len(grid[0])
        # find the first land
        si, sj = -1, -1
        for i in range(m):
            for j in range(n):
                if grid[i][j] == 1:
                    si, sj = i, j
                    break
            if (si, sj) != (-1, -1):
                break

        perimeter = 0
        visited = [
            [False for _ in range(n)]
            for __ in range(m)
        ]

        def dfs(x, y):
            nonlocal perimeter
            # (x,y) is not visited
            assert grid[x][y]
            visited[x][y] = True
            # check neighbors
            neighbors = [
                (x-1, y),
                (x+1, y),
                (x, y-1),
                (x, y+1)
            ]
            for nx, ny in neighbors:
                if 0 <= nx < m and 0 <= ny < n:
                    # neighbor is water
                    if grid[nx][ny] == 0:
                        perimeter += 1
                        continue
                    # unvisited land
                    elif not visited[nx][ny]:
                        dfs(nx, ny)
                else:  # out of border
                    perimeter += 1

        # start at (i,j)
        dfs(si, sj)
        return perimeter


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().islandPerimeter(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        [[0, 1, 0, 0],
         [1, 1, 1, 0],
            [0, 1, 0, 0],
            [1, 1, 0, 0]],
        16
    )

#
# @lc app=leetcode.cn id=63 lang=python3
#
# [63] 不同路径 II
#
from typing import List
# @lc code=start


class Solution:
    def uniquePathsWithObstacles(self, obstacleGrid: List[List[int]]) -> int:
        m, n = len(obstacleGrid), len(obstacleGrid[0])
        if obstacleGrid[0][0] == 1:
            return 0

        last_line = [0] * n
        line = [0] * n
        line[0] = 1
        for i in range(m):
            next_line = [0] * n
            for j in range(n):
                if obstacleGrid[i][j] == 1:
                    continue
                # (i, j) -> (i, j+1)
                if j+1 < n and obstacleGrid[i][j+1] != 1:
                    line[j+1] += line[j]

                # (i, j) -> (i+1, j)
                if i+1 < m and obstacleGrid[i+1][j] != 1:
                    next_line[j] = line[j]

            last_line, line = line, next_line
        return last_line[n-1]

    def uniquePathsWithObstacles_BFS(self, obstacleGrid: List[List[int]]) -> int:
        from queue import Queue

        m, n = len(obstacleGrid), len(obstacleGrid[0])
        if obstacleGrid[0][0] == 1:
            return 0
        f = [[0] * n for _ in range(m)]
        f[0][0] = 1
        q = Queue()
        q.put((0, 0))

        while q.qsize():
            x, y = q.get()
            if x+1 < m and obstacleGrid[x+1][y] == 0:
                if f[x+1][y] == 0:
                    q.put((x+1, y))
                f[x+1][y] += f[x][y]
            if y+1 < n and obstacleGrid[x][y+1] == 0:
                if f[x][y+1] == 0:
                    q.put((x, y+1))
                f[x][y+1] += f[x][y]
        # print(f)
        return f[m-1][n-1]


# @lc code=end
if __name__ == "__main__":
    f = Solution().uniquePathsWithObstacles
    assert f([
        [0, 0, 0],
        [0, 1, 0],
        [0, 0, 0]
    ]) == 2
    assert f([[1]]) == 0
    assert f([[0]]) == 1

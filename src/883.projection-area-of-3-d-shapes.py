#
# @lc app=leetcode id=883 lang=python3
#
# [883] Projection Area of 3D Shapes
#

# @lc code=start
class Solution:
    def projectionArea(self, grid: List[List[int]]) -> int:
        lenX, lenY = len(grid), len(grid[0])
        s = 0
        for x in range(lenX):
            for y in range(lenY):
                if grid[x][y]:
                    s += 1
            s += max(grid[x])
        for y in range(lenY):
            s += max(
                grid[x][y]
                for x in range(lenX)
            )
        return s
# @lc code=end


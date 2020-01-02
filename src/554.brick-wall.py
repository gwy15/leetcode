#
# @lc app=leetcode id=554 lang=python3
#
# [554] Brick Wall
#
from typing import *

# @lc code=start
from collections import defaultdict
class Solution:
    def leastBricks(self, wall: List[List[int]]) -> int:
        num = defaultdict(int)
        for bricks in wall:
            pos = 0
            for brick in bricks[:-1]:
                pos += brick
                num[pos] += 1
        if len(num) == 0:
            return len(wall)
        return len(wall) - max(num.values())
                    
        
# @lc code=end

Solution().leastBricks(
    [[1,2,2,1],
      [3,1,2],
      [1,3,2],
      [2,4],
      [3,1,2],
      [1,3,1,1]]
)

#
# @lc app=leetcode.cn id=672 lang=python3
#
# [672] 灯泡开关 Ⅱ
#

# @lc code=start
class Solution:
    def flipLights(self, lights: int, steps: int) -> int:
        # lights
        lights = min(lights, 3)
        # steps
        if steps == 0:
            return 1
        if steps == 1:
            return [2, 3, 4][lights - 1]
        if steps == 2:
            return [2, 4, 7][lights - 1]
        return [2, 4, 8][lights - 1]

# @lc code=end

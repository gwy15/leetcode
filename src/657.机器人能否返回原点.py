#
# @lc app=leetcode.cn id=657 lang=python3
#
# [657] 机器人能否返回原点
#

# @lc code=start
class Solution:
    def judgeCircle(self, moves: str) -> bool:
        x, y = 0, 0
        dx = {
            'U': 0, 'D': 0,
            'L': -1, 'R': 1
        }
        dy = {
            'L': 0, 'R': 0,
            'U': 1, 'D': -1
        }
        for ch in moves:
            x += dx[ch]
            y += dy[ch]
        return x == 0 and y == 0

# @lc code=end

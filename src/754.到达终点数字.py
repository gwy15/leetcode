#
# @lc app=leetcode.cn id=754 lang=python3
#
# [754] 到达终点数字
#

# @lc code=start
import math


class Solution:
    def reachNumber(self, target: int) -> int:
        t = abs(target)
        n = math.ceil(
            (math.sqrt(8 * t + 1) - 1) * .5
        )
        S = n * (n + 1) // 2
        if S % 2 == target % 2:
            return n
        elif (S + n + 1) % 2 == target % 2:
            return n+1
        else:
            return n+2


# @lc code=end
if __name__ == "__main__":
    f = Solution().reachNumber
    assert f(1) == 1
    assert f(3) == 2
    assert f(2) == 3
    assert f(28) == 7

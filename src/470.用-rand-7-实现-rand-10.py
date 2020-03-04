#
# @lc app=leetcode.cn id=470 lang=python3
#
# [470] 用 Rand7() 实现 Rand10()
#


def rand7():
    pass
# @lc code=start
# The rand7() API is already defined for you.
# @return a random integer in the range 1 to 7


class Solution:
    def rand10(self):
        """
        :rtype: int
        """
        BASE = 7
        TIMES = 3
        TARGET = 10
        MAX = BASE ** TIMES
        CLOSEST_RANGE = TARGET * (MAX // TARGET)
        # n in [0, BASE**TIMES)
        n = sum((BASE ** i) * (rand7() - 1) for i in range(TIMES))

        if n < CLOSEST_RANGE:
            return 1 + (n % TARGET)
        else:
            return self.rand10()


# @lc code=end

#
# @lc app=leetcode id=390 lang=python3
#
# [390] Elimination Game
#

# @lc code=start
class Solution:
    def lastRemaining(self, n: int) -> int:
        # start from left.
        if n <= 3:
            return [
               None, 1, 2, 2 
            ][n]
        # left (2n) = left (2n + 1)
        # = 1, 2, 3, ..., 2n, 2n+1
        # = _, 2, _, 4, ..., 2n
        # = 2 right(n)
        return 2 * self.lastRemaining2(n // 2)
    
    def lastRemaining2(self, n: int) -> int:
        if n <= 3:
            return [
                None, 1, 1, 2
            ][n]
        # right (2n)
        # = 1, 2, 3, ..., 2n-1, 2n
        # = 1, _, 3, ..., 2n-1, __
        # = (2, 4, 2n) - 1
        # = 2 left(n) - 1
        return 2 * self.lastRemaining(n // 2) - 1 + (n & 1)

# @lc code=end


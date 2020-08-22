#
# @lc app=leetcode.cn id=679 lang=python3
#
# [679] 24 点游戏
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def judgePoint24(self, nums: List[int]) -> bool:
        eps = 1e-6

        def dfs(rest) -> bool:
            if len(rest) == 1:
                return abs(rest[0] - 24) < eps
            # 选择
            for i, x in enumerate(rest):
                for j, y in enumerate(rest):
                    if i == j:
                        continue
                    new = [z for k, z in enumerate(rest) if k != i and k != j]
                    if i < j:
                        # +
                        new.append(x + y)
                        if dfs(new):
                            return True
                        new.pop()
                        # *
                        new.append(x * y)
                        if dfs(new):
                            return True
                        new.pop()
                    # -
                    new.append(x - y)
                    if dfs(new):
                        return True
                    new.pop()
                    # /
                    if abs(y) > eps:
                        new.append(x / y)
                        if dfs(new):
                            return True
            return False

        return dfs(nums)


# @lc code=end
if __name__ == "__main__":
    f = Solution().judgePoint24
    assert f([4, 1, 8, 7])
    assert not f([1, 2, 1, 2])

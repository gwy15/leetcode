#
# @lc app=leetcode.cn id=546 lang=python3
#
# [546] 移除盒子
#

from typing import List
# @lc code=start
from functools import lru_cache


class Solution:
    def removeBoxes(self, boxes: List[int]) -> int:
        @lru_cache(None)
        def dp(l, r, k) -> int:
            if r <= l:
                if l == r:
                    return (k + 1) ** 2
                else:
                    return 0

            # 6s => 0.8s
            while l < r and boxes[r-1] == boxes[r]:
                r -= 1
                k += 1

            ans = dp(l, r-1, 0) + (k + 1) ** 2
            ar = boxes[r]
            for i in range(l, r):
                if boxes[i] == ar:
                    ans = max(ans, dp(l, i, k+1) + dp(i+1, r-1, 0))

            return ans

        return dp(0, len(boxes)-1, 0)


# @lc code=end
if __name__ == "__main__":

    def test(i, ans):
        a = Solution().removeBoxes(i)
        print(f'{i} => {a}')
        assert a == ans
    test([1, 1, 1],  9)
    test([],  0)
    test([1, 2, 3],  3)
    test([1, 2, 1],  5)
    test([1, 3, 2, 2, 2, 3, 4, 3, 1],  23)
    test([1, 2, 1, 2, 1, 2, 1], 16 + 3)
    test([3, 8, 8, 5, 5, 3, 9, 2, 4, 4, 6, 5, 8, 4, 8, 6, 9, 6, 2, 8, 6, 4, 1, 9, 5, 3,
          10, 5, 3, 3, 9, 8, 8, 6, 5, 3, 7, 4, 9, 6, 3, 9, 4, 3, 5, 10, 7, 6, 10, 7], 136)
    test([1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
          1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2], 2550)

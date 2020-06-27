#
# @lc app=leetcode.cn id=1104 lang=python3
#
# [1104] 二叉树寻路
#
from typing import List
# @lc code=start


class Solution:
    def find_level(self, n: int) -> int:
        l = 0
        while n:
            n >>= 1
            l += 1
        return l

    def pathInZigZagTree(self, n: int) -> List[int]:
        ans = [n]
        l = self.find_level(n)
        while n != 1:
            # level starts at 2 ^ (l-1)
            i = n - (1 << (l - 1))
            # backtrace to up level
            _2_l_minus_2 = 1 << (l-2)
            j = _2_l_minus_2 - 1 - (i // 2)
            n = _2_l_minus_2 + j
            ans.append(n)

            l -= 1

        ans.reverse()
        return ans


# @lc code=end
if __name__ == "__main__":
    s = Solution()
    assert s.find_level(1) == 1
    assert s.find_level(2) == 2
    assert s.find_level(3) == 2
    assert s.find_level(4) == 3

    assert s.pathInZigZagTree(14) == [1, 3, 4, 14]
    assert s.pathInZigZagTree(26) == [1, 2, 6, 10, 26]
    assert s.pathInZigZagTree(1) == [1]
    assert s.pathInZigZagTree(2) == [1, 2]

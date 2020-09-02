#
# @lc app=leetcode.cn id=440 lang=python3
#
# [440] 字典序的第K小数字
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def findPrefixCount(self, prefix: int, n: int) -> int:
        """Get node count for trie tree (root included) in range [1, n]"""
        count = 0
        cur, next = prefix, prefix + 1
        # add by layer
        while cur <= n:
            count += min(next, n+1) - cur
            cur *= 10
            next *= 10

        return count

    def findKthNumber(self, n: int, k: int) -> int:
        ans = 1
        cnt = 1
        while cnt < k:
            tree = self.findPrefixCount(ans, n)
            if cnt + tree > k:
                ans *= 10
                cnt += 1
            else:
                ans += 1
                cnt += tree
        return ans

# @lc code=end


class Test(TestCase):
    def test_find_prefix_count(self):
        def f(p, n, ans): return self.assertEqual(
            Solution().findPrefixCount(p, n), ans)
        f(1, 12, 4)
        f(1, 1234, 346)
        f(12, 12, 1)

    def test_solution(self):
        def f(n, k, ans):
            self.assertEqual(Solution().findKthNumber(n, k), ans)
        f(13, 2, 10)


if __name__ == "__main__":
    testmain()

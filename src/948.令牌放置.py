#
# @lc app=leetcode.cn id=948 lang=python3
#
# [948] 令牌放置
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def bagOfTokensScore(self, tokens: List[int], P: int) -> int:
        tokens.sort()
        n = len(tokens)
        #
        left, right = 0, n-1
        score = 0
        while left <= right:
            while left <= right and P >= tokens[left]:
                P -= tokens[left]
                score += 1
                left += 1
            # if there are <= 2 tokens, exit
            if right - left <= 1 or score <= 0:
                break
            else:
                P += tokens[right]
                score -= 1
                right -= 1
        return score


# @lc code=end
if __name__ == '__main__':
    def test(input, P, expected):
        calc = Solution().bagOfTokensScore(input, P)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([100], 50, 0)
    test([100, 200], 150, 1)
    test([100, 200, 300, 400], 200, 2)
    test([71, 55, 82], 54, 0)

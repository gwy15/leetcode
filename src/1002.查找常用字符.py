#
# @lc app=leetcode.cn id=1002 lang=python3
#
# [1002] 查找常用字符
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def commonChars(self, A: List[str]) -> List[str]:
        def count(word):
            c = [0 for _ in range(26)]
            for ch in word:
                c[ord(ch) - ord('a')] += 1
            return c

        c = [float('inf')] * 26
        for word in A:
            c1 = count(word)
            for i in range(26):
                c[i] = min(c[i], c1[i])
        ans = []
        for i, cnt in enumerate(c):
            for _ in range(cnt):
                ans.append(chr(ord('a') + i))
        return ans

# @lc code=end

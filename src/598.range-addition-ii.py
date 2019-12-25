#
# @lc app=leetcode id=598 lang=python3
#
# [598] Range Addition II
#
class Solution:
    def maxCount(self, m: int, n: int, ops: List[List[int]]) -> int:
        if len(ops) == 0:
            return m * n
        mina = min(item[0] for item in ops)
        minb = min(item[1] for item in ops)
        return mina * minb


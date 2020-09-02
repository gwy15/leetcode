#
# @lc app=leetcode.cn id=1389 lang=python3
#
# [1389] 按既定顺序创建目标数组
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def createTargetArray(self, nums: List[int], index: List[int]) -> List[int]:
        ans = []
        for i, n in zip(index, nums):
            ans.insert(i, n)

        return ans

# @lc code=end

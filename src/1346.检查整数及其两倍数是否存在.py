#
# @lc app=leetcode.cn id=1346 lang=python3
#
# [1346] 检查整数及其两倍数是否存在
#
from typing import List
from utils import *
# @lc code=start
class Solution:
    def checkIfExist(self, arr: List[int]) -> bool:
        seen = set()
        for n in arr:
            if n in seen:
                return True
            seen.add(n * 2)
            if n % 2 == 0:
                seen.add(n // 2)
        return False


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().checkIfExist(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([10, 2, 5, 3], True)
    test([7, 1, 14, 11], True)
    test([3, 1, 7, 11], False)

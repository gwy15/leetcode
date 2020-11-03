#
# @lc app=leetcode.cn id=941 lang=python3
#
# [941] 有效的山脉数组
#
from prelude import *
# @lc code=start

from enum import Enum


class Status(Enum):
    Empty = 0
    Init = 1
    Asc = 2
    Desc = 3


class Solution:
    def validMountainArray(self, A: List[int]) -> bool:
        status = Status.Empty
        prev = 0
        for n in A:
            if status == Status.Empty:
                status = Status.Init
            elif status == Status.Init:
                if n <= prev:
                    return False
                status = Status.Asc
            elif status == Status.Asc:
                if n < prev:
                    status = Status.Desc
                elif n == prev:
                    return False

            else:
                # desc
                if n >= prev:
                    return False
            prev = n
        return status == Status.Desc


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().validMountainArray(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([2, 1], False)
    test([3, 5, 5], False)
    test([0, 3, 2, 1], True)
    test([1, 2], False)
    test([1, 2, 3], False)
    test([3, 2], False)

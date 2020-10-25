#
# @lc app=leetcode.cn id=845 lang=python3
#
# [845] 数组中的最长山脉
#
from typing import List
from utils import *
# @lc code=start

import enum


class Status(enum.Enum):
    UNSET = 0
    ASC = 1
    DESC = 2


class Solution:
    def longestMountain(self, A: List[int]) -> int:
        n = len(A)
        if n < 3:
            return 0
        status = Status.UNSET
        i = 1
        last = A[0]
        start_index = 0
        max_length = 0
        while i < n:
            a = A[i]
            # print(f'{a=},  {last =}, last {status=}')

            if status == Status.UNSET:
                if a > last:
                    status = Status.ASC
                else:  # <=
                    status = Status.UNSET
                    start_index = i

            elif status == Status.ASC:
                if a > last:
                    status = Status.ASC
                elif a < last:
                    status = Status.DESC
                    max_length = max(max_length, i - start_index + 1)
                else:  # ==
                    status = Status.UNSET
                    start_index = i

            else:  # DESC
                if a > last:
                    start_index = i - 1
                    status = Status.ASC
                elif a == last:
                    status = Status.UNSET
                    start_index = i
                else:  # <
                    status = Status.DESC
                    max_length = max(max_length, i - start_index + 1)
            # print(f'  after: start={A[start_index]}, {max_length=}, {status=}')
            last = a
            i += 1

        return max_length


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().longestMountain(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([], 0)
    test([1, 2], 0)
    test([2, 2, 2], 0)
    test([1, 2, 1], 3)
    test([2, 1, 4, 7, 3, 2, 5], 5)
    test([875, 884, 239, 731, 723, 685], 4)

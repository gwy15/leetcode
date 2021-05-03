#
# @lc app=leetcode.cn id=1376 lang=python3
#
# [1376] 通知所有员工所需的时间
#
from prelude import *
# @lc code=start


class Solution:
    def numOfMinutes(self, n: int, headID: int, manager: List[int], informTime: List[int]) -> int:
        time = {}

        def find(i):
            nonlocal time
            if i == -1:
                return 0
            if i in time:
                return time[i]
            man = manager[i]
            # manager 知道时的时间
            man_t = find(man)
            if man >= 0:
                t = man_t + informTime[man]
            else:
                t = man_t
            time[i] = t
            return t

        for i in range(n):
            find(i)
        return max(time.values())


# @lc code=end
if __name__ == '__main__':
    def test(n, headID, manager, informTime, expected):
        calc = Solution().numOfMinutes(n, headID, manager, informTime)
        if calc != expected:
            print(f'case failed: `{n, headID, manager, informTime}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        n=7, headID=6, manager=[1, 2, 3, 4, 5, 6, -1], informTime=[0, 6, 5, 4, 3, 2, 1],
        expected=21
    )
    test(
        n=15, headID=0, manager=[-1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6], informTime=[1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        expected=3
    )
    test(
        n=4, headID=2, manager=[3, 3, -1, 2], informTime=[0, 0, 162, 914],
        expected=1076
    )

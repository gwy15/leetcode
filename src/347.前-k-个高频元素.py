#
# @lc app=leetcode.cn id=347 lang=python3
#
# [347] 前 K 个高频元素
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def get_num_and_freq(self, nums: List[int]) -> List:
        from collections import defaultdict
        times = defaultdict(int)
        for n in nums:
            times[n] += 1
        return times.items()

    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        from queue import PriorityQueue
        q = PriorityQueue(0)
        for (num, times) in self.get_num_and_freq(nums):
            if q.qsize() < k:
                q.put((times, num))
            else:
                (times_old, num_old) = q.get()
                if times_old > times:
                    num, times = num_old, times_old
                q.put((times, num))
        ans = []
        for i in range(k):
            ans.append(q.get()[1])
        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, k, expected):
        calc = Solution().topKFrequent(input, k)
        if set(calc) != set(expected):
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 1, 1, 2, 2, 3], 2, [1, 2])
    test([1, 2, 3, 4, 5, 6, 7], 7, [1, 2, 3, 4, 5, 6, 7])
    test([1], 1, [1])

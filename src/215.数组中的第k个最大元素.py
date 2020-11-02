#
# @lc app=leetcode.cn id=215 lang=python3
#
# [215] 数组中的第K个最大元素
#
from prelude import *
# @lc code=start
from typing import Tuple
import random


class Solution:
    def partition(self, nums: List[int], i: int, j: int, pivot: int) -> Tuple[int, int, int]:
        less, equal, greater = i, i, j
        # [i, less) => less
        # [less, equal) => equal
        # [greater, j) => greater
        while equal < greater:
            if nums[equal] == pivot:
                equal += 1
            elif nums[equal] < pivot:
                # swap [less], [equal]
                nums[less], nums[equal] = nums[equal], nums[less]
                less += 1
                equal += 1
            else:
                # >
                greater -= 1
                nums[equal], nums[greater] = nums[greater], nums[equal]

        # 返回 < = > 数量
        return (less - i, equal - less, j - greater)

    def findKthLargestInSlice(self, nums: List[int], i: int, j: int, k: int) -> int:
        if i + 1 == j:
            return nums[i]
        # [i, j)
        pivot = nums[random.randrange(i, j)]
        less_cnt, equal_cnt, greater_cnt = self.partition(nums, i, j, pivot)
        # 在 > 段
        if greater_cnt >= k:
            return self.findKthLargestInSlice(nums, i+less_cnt+equal_cnt, j, k)
        if greater_cnt + equal_cnt >= k:
            return pivot
        else:  # < 段
            return self.findKthLargestInSlice(nums, i, i+less_cnt, k-greater_cnt-equal_cnt)

    def findKthLargest(self, nums: List[int], k: int) -> int:
        return self.findKthLargestInSlice(nums, 0, len(nums), k)


# @lc code=end
if __name__ == '__main__':
    def test(input, k, expected):
        calc = Solution().findKthLargest(input[::], k)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([3, 2, 1, 5, 6, 4], 2, 5)
    test([3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4)

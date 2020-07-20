#
# @lc app=leetcode.cn id=167 lang=python3
#
# [167] 两数之和 II - 输入有序数组
#
from typing import List
# @lc code=start


class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        n = len(numbers)
        i, j = 0, n-1
        while i < j:
            if numbers[i] + numbers[j] < target:
                left = numbers[i]
                while i < j and numbers[i] == left:
                    i += 1
            elif numbers[i] + numbers[j] == target:
                return [i + 1, j + 1]
            else:
                right = numbers[j]
                while i < j and numbers[j] == right:
                    j -= 1

        raise ReferenceError


# @lc code=end
if __name__ == "__main__":
    f = Solution().twoSum
    assert f([2, 7, 11, 15], 9) == [1, 2]
    assert f([1, 2], 3) == [1, 2]

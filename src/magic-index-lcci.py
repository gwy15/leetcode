from typing import List


class Solution:
    def findMagicIndex(self, nums: List[int]) -> int:
        for i, n in enumerate(nums):
            if i == n:
                return i
        return -1


if __name__ == "__main__":
    f = Solution().findMagicIndex
    assert f([0, 2, 3, 4, 5]) == 0
    assert f([1, 1, 1]) == 1
    assert f([]) == -1
    assert f([1]) == -1
    assert f([0]) == 0
    assert f([1, 1, 2, 3]) == 1

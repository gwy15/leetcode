from prelude import *


class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        cur = None
        count = 0
        for n in nums:
            if cur == None:
                cur = n
                count = 1
            elif cur == n:
                count += 1
            else:  # !=
                count -= 1
                if count == 0:
                    cur = None
        if count > 0:
            # 二次验证
            count = sum(n == cur for n in nums)
            if count > (len(nums) // 2):
                return cur
        return -1


if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().majorityElement(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 2, 5, 9, 5, 9, 5, 5, 5], 5)
    test([3, 2], -1)
    test([2, 2, 1, 1, 1, 2, 2], 2)
    test([1], 1)
    test([1, 1, 2], 1)
    test([1, 1, 2, 2], -1)
    test([1, 2, 3], -1)

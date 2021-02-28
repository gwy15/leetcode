#
# @lc app=leetcode.cn id=628 lang=python3
#
# [628] 三个数的最大乘积
#
from prelude import *
# @lc code=start
class Solution:
    def maximumProduct(self, nums: List[int]) -> int:
        # max, min
        one = []
        two = []
        three = []
        for n in nums:
            if len(two):
                if three:
                    three[0] = max(three[0], two[0] * n, two[1] * n)
                    three[1] = min(three[1], two[0] * n, two[1] * n)
                else:
                    three = [
                        max(two[0] * n, two[1] * n),
                        min(two[0] * n, two[1] * n)
                    ]
            
            if len(one):
                if two:
                    two[0] = max(two[0], one[0] * n, one[1] * n )
                    two[1] = min(two[1], one[0] * n, one[1] * n )
                else:
                    two = [max(one[0] * n, one[1] * n), min(one[0] * n, one[1] * n)]

            if len(one):
                one[0] = max(one[0], n)
                one[1] = min(one[1], n)
            else:
                one = [n, n]
            
        return three[0]

# @lc code=end

if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().maximumProduct(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1,2,3], 6)
    test([-1, -2, -3], -6)
    test([1,2,3,4], 24)

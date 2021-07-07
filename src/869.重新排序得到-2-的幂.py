#
# @lc app=leetcode.cn id=869 lang=python3
#
# [869] 重新排序得到 2 的幂
#
from prelude import *
# @lc code=start

import itertools

# # 1, ... 2**30 = 1073741824
# powerOfTwos = set(2 ** i for i in range(31))


class Solution:
    # def isPowerOfTwo(self, n: int) -> bool:
    #     return n in powerOfTwos

    def reorderedPowerOf2(self, n: int) -> bool:
        digits = list(i for i in str(n))
        digits = sorted(digits)
        for k in range(31):
            currect_digits = list(i for i in str(2 ** k))
            currect_digits = sorted(currect_digits)
            if currect_digits == digits:
                return True
        return False

        # n <= 10^9, 位数 <= 9，最多 9 位，排列组合也就 9! =36w，可以接受
        # digits = list(int(i) for i in str(n))
        # for perm in itertools.permutations(digits):
        #     s = ''.join(str(i) for i in perm)
        #     if s.startswith('0'):
        #         continue
        #     if self.isPowerOfTwo(int(s)):
        #         return True
        # return False
# @lc code=end

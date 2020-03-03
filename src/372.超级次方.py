#
# @lc app=leetcode.cn id=372 lang=python3
#
# [372] 超级次方
#

# @lc code=start
import math
MOD = 1337
PHI_MOD = 1140


class Solution:
    def mod_list(self, b: List[int], m: int) -> int:
        res = 0
        for i in b:
            if res > 2000_0000:
                res = (10 * res + i) % m
            else:
                res = 10 * res + i
        return res % m

    def mod_list_with_subtract(self, a: List[int], b: int, m: int) -> int:
        """(a - b) % m"""
        return (self.mod_list(a, m) - b) % m

    def superPow(self, a: int, b: List[int]) -> int:
        a = a % MOD
        # b = int(''.join(str(c) for c in b))
        # return pow(a, b, mod=MOD)

        # 欧拉定理
        if math.gcd(a, MOD) == 1:  # 互质
            b = self.mod_list(b, PHI_MOD)
            return (a ** b) % MOD
        # 寻找环的交叉点
        mod_to_index = {}
        x = a
        for i in range(MOD):
            if x in mod_to_index:
                cross_i = mod_to_index[x]
                loop_length = i - cross_i
                # b = cross_i + (b - cross_i) % loop_length
                b = cross_i + \
                    self.mod_list_with_subtract(b, cross_i, loop_length)
                break
            mod_to_index[x] = i
            x = (x * a) % MOD
        return (a ** b) % MOD


# @lc code=end

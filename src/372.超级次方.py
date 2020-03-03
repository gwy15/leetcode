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
    def superPow(self, a: int, b: List[int]) -> int:
        a = a % MOD
        # b = int(''.join(str(c) for c in b))
        bb = 0
        for _b in b:
            bb = 10 * bb + _b
        b = bb
        # return pow(a, b, mod=MOD)
        
        # 欧拉定理
        if math.gcd(a, MOD) == 1: # 互质
            b = b % PHI_MOD
            return (a ** b) % MOD
        # 简单情况直接计算
        if b < MOD:
            return (a ** b) % MOD
        # 寻找环的交叉点
        mod_to_index = {}
        x = a
        for i in range(MOD):
            if x in mod_to_index:
                cross_i = mod_to_index[x]
                loop_length = i - cross_i
                b = cross_i + (b - cross_i) % loop_length
                break
            mod_to_index[x] = i
            x = (x * a) % MOD
        return (a ** b) % MOD


# @lc code=end


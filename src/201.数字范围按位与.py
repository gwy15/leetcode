#
# @lc app=leetcode.cn id=201 lang=python3
#
# [201] 数字范围按位与
#

# @lc code=start
class Solution:
    def rangeBitwiseAnd(self, m: int, n: int) -> int:
        # 寻找二进制的公共前缀
        for i in range(32):
            if m == n:
                return m << i
            m >>= 1
            n >>= 1


# @lc code=end
if __name__ == "__main__":
    f = Solution().rangeBitwiseAnd
    assert f(3, 4) == 0
    assert f(5, 7) == 4
    assert f(0, 1) == 0
    assert f(0b010101, 0b101010) == 0
    assert f(0b101010, 0b110011) == 0b100000
    assert f(0b110011, 0b111010) == 0b110000

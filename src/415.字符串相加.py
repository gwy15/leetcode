#
# @lc app=leetcode.cn id=415 lang=python3
#
# [415] 字符串相加
#

# @lc code=start
class Solution:
    def digits(self, s):
        i, n = 0, len(s)
        while i < n:
            yield int(s[n - 1 - i])
            i += 1

    def addStrings(self, num1: str, num2: str) -> str:
        n1, n2 = len(num1), len(num2)
        digits1 = self.digits(num1)
        digits2 = self.digits(num2)
        i, j, rest = 0, 0, 0
        digits = []

        while i < len(num1) or j < len(num2) or rest:
            rest += next(digits1, 0) + next(digits2, 0)
            digits.append(rest % 10)
            i += 1
            j += 1
            rest //= 10
        while len(digits) > 0 and digits[-1] == 0:
            digits.pop()
        
        if len(digits) == 0:
            digits = [0]
        n = len(digits)
        return ''.join(str(digits[n-1-i]) for i in range(n))


# @lc code=end
if __name__ == "__main__":
    f = Solution().addStrings
    assert f('0', '0') == '0'
    assert f('1', '2') == '3'
    assert f('999', '1') == '1000'
    assert f('123', '456') == '579'
#
# @lc app=leetcode.cn id=43 lang=python3
#
# [43] 字符串相乘
#

# @lc code=start
from typing import List
import math


class Solution_FFT:
    def change(self, y: List[complex]):
        rev = [0] * len(y)
        for i in range(len(y)):
            rev[i] = rev[i >> 1] >> 1
            if i & 1:
                rev[i] |= len(y) >> 1
        for i in range(len(y)):
            if i < rev[i]:
                y[i], y[rev[i]] = y[rev[i]], y[i]

    def fft(self, y: List[complex], on: int):
        """Inplace FFT/IFFT"""
        sin, cos, pi = math.sin, math.cos, math.pi
        l = len(y)
        self.change(y)
        h = 2
        while h <= l:
            wn = complex(cos(2 * pi / h), sin(on * 2 * pi / h))
            j = 0
            while j < l:
                w = complex(1, 0)
                for k in range(j, j + h // 2):
                    u = y[k]
                    t = w * y[k + h // 2]
                    y[k] = u + t
                    y[k + h // 2] = u - t
                    w *= wn
                j += h
            h <<= 1
        if on == -1:
            for i in range(l):
                y[i] /= l

    def multiply(self, num1: str, num2: str) -> str:
        n1, n2 = [int(i) for i in num1], [int(i) for i in num2]
        l = len(n1) + len(n2)
        if l != 1 << (l.bit_length() - 1):
            l = 1 << l.bit_length()
        #
        n1, n2 = [complex(i) for i in n1[::-1]], [complex(i) for i in n2[::-1]]
        # make paddings
        n1 += [complex(0) for i in range(l - len(n1))]
        n2 += [complex(0) for i in range(l - len(n2))]
        #
        self.fft(n1, 1)
        self.fft(n2, 1)
        ans = [n1[i] * n2[i] for i in range(len(n1))]
        self.fft(ans, -1)
        ans = [int(i.real + 0.3) for i in ans]
        last = 0
        for i in range(len(ans)):
            digit = ans[i] + last
            last, digit = digit // 10, digit % 10
            ans[i] = digit
        i = len(ans) - 1
        while i > 0 and ans[i] == 0:
            i -= 1
        ans = ''.join(str(i) for i in ans[:i+1])[::-1]
        return ans


class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        if num1 == '0' or num2 == '0':
            return '0'
        elif num1 == '1' or num2 == '1':
            return num1 if num2 == '1' else num2
        n1, n2 = [int(i) for i in num1], [int(i) for i in num2]
        ans = [0] * (len(n1) + len(n2))
        for i in range(len(n2)):
            b = n2[len(n2) - 1 - i]
            # ans += a * b
            j, last = 0, 0
            while j < len(n1) or last > 0:
                digit = ans[i + j] + last
                if j < len(n1):
                    digit += n1[len(n1) - 1 - j] * b

                last, digit = digit // 10, digit % 10
                ans[i + j] = digit
                j += 1
        # pop [0, i]
        i = len(ans) - 1
        while i > 0 and ans[i] == 0:
            i -= 1
        return ''.join(str(i) for i in ans[:i+1])[::-1]


# @lc code=end
if __name__ == "__main__":
    def test(a, b):
        sa = str(a)
        sb = str(b)
        sans = Solution().multiply(sa, sb)
        assert sans == str(a * b)

    test(9, 9)
    test(123456, 654321)
    test(1, 0)
    test(0, 0)
    test(1111111111, 1111111111)
    test(121111111111111, 0)

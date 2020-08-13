#
# @lc app=leetcode.cn id=43 lang=python3
#
# [43] 字符串相乘
#

# @lc code=start
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

#
# @lc app=leetcode.cn id=984 lang=python3
#
# [984] 不含 AAA 或 BBB 的字符串
#

# @lc code=start
class Solution:
    def strWithout3a3b(self, a: int, b: int) -> str:
        s = []
        while a > 0 or b > 0:
            if a == 0:
                s.append('b' * b)
                break
            if b == 0:
                s.append('a' * a)
                break
            if a > b:
                s.append('aa')
                a -= 2
                if b > 0:
                    s.append('b')
                    b -= 1
            elif b > a:
                s.append('bb')
                b -= 2
                if a > 0:
                    s.append('a')
                    a -= 1
            else:
                s.append('a')
                a -= 1

        return ''.join(s)
# @lc code=end


if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().strWithout3a3b(*input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 2], 'bba')
    test([4, 1], 'aabaa')
    test([2, 2], 'abba')
    test([0, 0], '')
    test([3, 3], 'abbaab')

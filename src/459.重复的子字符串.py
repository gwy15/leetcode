#
# @lc app=leetcode.cn id=459 lang=python3
#
# [459] 重复的子字符串
#

# @lc code=start
class Solution:
    def repeatedSubstringPattern(self, s: str) -> bool:
        n = len(s)
        for sub_len in range(1, n):
            if n % sub_len != 0:
                continue
            times = n // sub_len
            matched = True
            for i in range(sub_len, n):
                if s[i] != s[i % sub_len]:
                    matched = False
                    break
            if matched:
                return True
        return False


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().repeatedSubstringPattern(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('abab', True)
    test('aba', False)
    test('bb', True)
    test('abcabcabc', True)

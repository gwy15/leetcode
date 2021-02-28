#
# @lc app=leetcode.cn id=1657 lang=python3
#
# [1657] 确定两个字符串是否接近
#

# @lc code=start

class Solution:
    def closeStrings(self, word1: str, word2: str) -> bool:
        # 只要所有字符的出现次数排序一致就可以
        # 而且出现的字符集一样
        def helper(s):
            cnt = {}
            for ch in s:
                cnt[ch] = cnt.get(ch, 0) + 1
            return sorted(cnt.keys()), sorted(cnt.values())
        return helper(word1) == helper(word2)


# @lc code=end
if __name__ == '__main__':
    def test(s1, s2, expected):
        calc = Solution().closeStrings(s1, s2)
        if calc != expected:
            print(f'case failed: `{s1, s2}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('abc', 'bca', True)
    test('a', 'aa', False)
    test('cabbba', 'abbccc', True)
    test('cabbba', 'aabbss', False)
    test('aab', 'ccb', False)

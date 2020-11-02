#
# @lc app=leetcode.cn id=76 lang=python3
#
# [76] 最小覆盖子串
#

# @lc code=start
from collections import Counter


class Solution:
    def minWindow(self, s: str, t: str) -> str:
        n = len(s)
        t_chars = Counter(t)
        todo_char_count = len(t_chars)
        best_i, best_j = 0, 2*n
        # [i, j)
        i, j = 0, 0

        while i <= j and j < n:
            ch = s[j]
            j += 1

            if ch not in t_chars:
                # 不关心的字符
                continue
            # s[j] 在字符串 t 中，需要考虑

            t_chars[ch] -= 1
            if t_chars[ch] > 0:
                # 没满足这个字符，继续
                continue
            # 刚好把这个字符满足
            if t_chars[ch] == 0:
                todo_char_count -= 1

            if todo_char_count > 0:
                # 没满足全部字符，继续
                continue
            # 刚好全部清 0，开始增加左侧窗口

            while i < j and todo_char_count <= 0:
                # 记录当前的位置
                if j - i < best_j - best_i:
                    best_i, best_j = i, j

                ch_i = s[i]
                i += 1
                # 减去 [i] 字符
                if ch_i not in t_chars:
                    # 不关心的字符
                    continue
                t_chars[ch_i] += 1
                if t_chars[ch_i] <= 0:
                    # 还是满足
                    continue
                # 不是全部字符都满足了
                todo_char_count += 1

        if best_j == 2 * n:
            return ''
        return s[best_i:best_j]


# @lc code=end
if __name__ == '__main__':
    def test(s, t, expected):
        calc = Solution().minWindow(s, t)
        if calc != expected:
            print(f'case failed: `{(s, t)}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test("ADOBECODEBANC", "ABC", "BANC")

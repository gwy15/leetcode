#
# @lc app=leetcode.cn id=1410 lang=python3
#
# [1410] HTML 实体解析器
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def entityParser(self, text: str) -> str:
        text = list(text)
        i, n = 0, len(text)
        ans = []
        while i < n:
            ch = text[i]
            if ch == '&':
                def f(s, t):
                    nonlocal i
                    l = len(s)
                    if ''.join(text[i+1:i+1+l]) == s:
                        ans.append(t)
                        i += 1 + l
                        return True
                    return False

                if f('quot;', '"'):
                    continue
                if f('apos;', "'"):
                    continue
                if f('amp;', '&'):
                    continue
                if f('gt;', '>'):
                    continue
                if f('lt;', '<'):
                    continue
                if f('frasl;', '/'):
                    continue
            ans.append(ch)
            i += 1
        return ''.join(ans)


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().entityParser(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test("&amp; is an HTML entity but &ambassador; is not.",
         "& is an HTML entity but &ambassador; is not.")
    test("and I quote: &quot;...&quot;", "and I quote: \"...\"")
    test("Stay home! Practice on Leetcode :)",
         "Stay home! Practice on Leetcode :)")
    test("x &gt; y &amp;&amp; x &lt; y is always false",
         "x > y && x < y is always false")
    test(
        "leetcode.com&frasl;problemset&frasl;all", "leetcode.com/problemset/all"
    )

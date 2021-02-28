#
# @lc app=leetcode.cn id=32 lang=python3
#
# [32] 最长有效括号
#
from prelude import *
# @lc code=start


class Solution:
    def longestValidParentheses(self, s: str) -> int:
        stack = [-1]
        best = 0
        for i, ch in enumerate(s):
            if ch == '(':
                stack.append(i)
            else:
                stack.pop()
                if len(stack) > 0:
                    best = max(best, i - stack[-1])
                else:
                    # illegal start at i
                    stack.append(i)
        return best


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().longestValidParentheses(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('(()', 2)
    test(')()())', 4)
    test('', 0)

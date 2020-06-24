#
# @lc app=leetcode.cn id=20 lang=python3
#
# [20] 有效的括号
#

# @lc code=start


class Solution:
    def isValid(self, s: str) -> bool:
        pairs = {
            '(': ')',
            '[': ']',
            '{': '}'
        }
        stack = []
        for ch in s:
            if ch in '([{':
                stack.append(ch)
            else:
                if len(stack) == 0:
                    return False
                left = stack.pop()
                if ch != pairs[left]:
                    return False
        return len(stack) == 0


# @lc code=end
if __name__ == "__main__":
    s = Solution()
    f = s.isValid
    assert f('()')
    assert not f('()(')
    assert f('()[]{}')
    assert not f('([)]')
    assert f('{[]}')

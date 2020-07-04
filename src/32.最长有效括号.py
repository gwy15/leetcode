#
# @lc app=leetcode.cn id=32 lang=python3
#
# [32] 最长有效括号
#

# @lc code=start


class Solution:
    def longestValidParentheses_greedy(self, s: str) -> int:
        ans = 0

        left, right = 0, 0
        for ch in s:
            if ch == '(':
                left += 1
            else:
                right += 1

            if left == right:
                ans = max(ans, 2 * right)
            elif left < right:
                left, right = 0, 0

        left, right = 0, 0
        for ch in s[::-1]:
            if ch == '(':
                left += 1
            else:
                right += 1

            if left == right:
                ans = max(ans, 2 * right)
            elif left > right:
                left, right = 0, 0

        return ans

    def longestValidParentheses_stack(self, s: str) -> int:
        ans = 0

        # stack looks like ')((((' (for index)
        stack = []
        stack.append(-1)
        for idx, ch in enumerate(s):
            if ch == '(':
                stack.append(idx)
            else:  # ')'
                # stack is not empty
                stack.pop()
                if len(stack) == 0:  # popped last )
                    # make this the new )
                    stack.append(idx)
                else:
                    top = stack[-1]
                    ans = max(ans, idx - top)

        return ans

    def longestValidParentheses(self, s: str) -> int:
        # return self.longestValidParentheses_greedy(s)
        return self.longestValidParentheses_stack(s)


# @lc code=end
if __name__ == "__main__":
    f = Solution().longestValidParentheses
    assert f('(()') == 2
    assert f(')()())') == 4
    assert f('') == 0
    assert f(')(') == 0
    assert f('(((((((((((((((((()') == 2
    assert f('))))))))((())))') == 6
    assert f('()(()') == 2

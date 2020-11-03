#
# @lc app=leetcode.cn id=301 lang=python3
#
# [301] 删除无效的括号
#
from prelude import *
# @lc code=start


class Solution:
    def removeInvalidParentheses(self, s: str) -> List[str]:
        def isValid(s: str) -> bool:
            cnt = 0
            for c in s:
                if c == "(":
                    cnt += 1
                elif c == ")":
                    cnt -= 1
                if cnt < 0:
                    return False
            return cnt == 0

        # BFS
        level = {s}  # 用set避免重复
        while True:
            filtered = list(filter(isValid, level))
            if len(filtered) > 0:
                return filtered
            # 下一层level
            next_level = set()
            for item in level:
                for i in range(len(item)):
                    if item[i] in "()":
                        next_level.add(item[:i]+item[i+1:])
            level = next_level


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().removeInvalidParentheses(input)
        if set(calc) != set(expected):
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('()', ['()'])
    test("()())()", ["()()()", "(())()"])
    test("(a)())()", ["(a)()()", "(a())()"])
    test(")(", [""])

#
# @lc app=leetcode.cn id=140 lang=python3
#
# [140] 单词拆分 II
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        n = len(s)
        words = set(wordDict)
        # cached, solutions[i] = [0, i) 解法
        solutions = [
            None for _ in range(n+1)
        ]
        solutions[0] = [[]]

        def dfs(i):  # i in [1, n], 判断 [0, i) 子串
            if solutions[i] is not None:
                return solutions[i]
            ans = []

            for start in range(0, i):  # 判断 [start, i) 是否是单词
                possible_word = s[start:i]
                if possible_word in words:
                    prev_solutions = dfs(start)
                    for prev_solution in prev_solutions:
                        ans.append(prev_solution + [possible_word])

            solutions[i] = ans
            return ans

        dfs(n)
        return [' '.join(words) for words in solutions[n]]


# @lc code=end
if __name__ == '__main__':
    def test(s, wordDict, expected):
        calc = Solution().wordBreak(s, wordDict)
        if set(calc) != set(expected):
            print(f'case failed: `{s}, {wordDict}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('catsanddog', ["cat", "cats", "and", "sand", "dog"], [
        "cats and dog",
        "cat sand dog"
    ])
    test("pineapplepenapple", ["apple", "pen", "applepen", "pine", "pineapple"], [
        "pine apple pen apple",
        "pineapple pen apple",
        "pine applepen apple"
    ])
    Solution().wordBreak(
        "aaaaaaaaaaaaaaaaaaa",
        ["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa",
            "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"]
    )
    test(
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa",
            "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"],
        ''
    )
    test(
        'a', [], ''
    )

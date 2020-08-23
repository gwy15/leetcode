#
# @lc app=leetcode.cn id=784 lang=python3
#
# [784] 字母大小写全排列
#
from typing import List
from utils import *
# @lc code=start


class Solution:
    def letterCasePermutation(self, S: str) -> List[str]:
        S = list(S)
        alpha_indexes = [i for i, ch in enumerate(S) if ch.isalpha()]
        num_alphas = len(alpha_indexes)
        ans = []
        #
        for i in range(1 << num_alphas):
            # make a copy
            chars = S[::]
            for index_in_alpha, index_in_s in enumerate(alpha_indexes):
                ch = chars[index_in_s]
                if (i >> index_in_alpha) & 1:
                    ch = ch.lower()
                else:
                    ch = ch.upper()
                chars[index_in_s] = ch
            ans.append(''.join(chars))

        return ans


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().letterCasePermutation(input)
        if set(calc) != set(expected):
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('a1b2', ["a1b2", "a1B2", "A1b2", "A1B2"])
    test('3z4', ["3z4", "3Z4"])
    test("12345", ['12345'])

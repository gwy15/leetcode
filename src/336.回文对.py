#
# @lc app=leetcode.cn id=336 lang=python3
#
# [336] 回文对
#
from typing import List
# @lc code=start


class Solution:
    def palindromePairs(self, words: List[str]) -> List[List[int]]:
        n = len(words)
        res = []

        def is_palindrome(s):
            return s == s[::-1]

        s_index = {}

        # 预处理
        for i in range(n):
            s_index[words[i]] = i

        for i, s in enumerate(words):
            l = len(s)
            for pos in range(l+1):
                left, right = s[:pos], s[pos:]

                if is_palindrome(left):
                    # ____, <left>, <right>
                    # left 可为空 => j,i 长度相等
                    # right 可为空 => s 自身回文+空串
                    j = s_index.get(right[::-1], None)
                    if j is not None:
                        # print(f'match: {s=:8}, {words[j]=:8}, {left=:8}, {right=:8}')
                        if i != j:
                            res.append([j, i])

                if pos < l and is_palindrome(right):
                    # <left>,  <right>, ___
                    # left 为空 => 自身回文
                    # right 不允许为空
                    j = s_index.get(left[::-1], None)
                    if j is not None:
                        # print(f'match: {s=:8}, {left=:12}, {right=:7}, {words[j]=:8}')
                        res.append([i, j])

        # print(res)
        return res

# @lc code=end


if __name__ == "__main__":
    f = Solution().palindromePairs
    assert f(["abcd", "dcba", "lls", "s", "sssll"]) == [
        [1, 0], [0, 1], [3, 2], [2, 4]
    ]
    assert f(["bat", "tab", "cat"]) == [[1, 0], [0, 1]]
    assert f(['a', '']) == [[0, 1], [1, 0]]

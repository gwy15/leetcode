#
# @lc app=leetcode.cn id=1218 lang=python3
#
# [1218] 最长定差子序列
#
from prelude import *
# @lc code=start


class Solution:
    def longestSubsequence(self, arr: List[int], difference: int) -> int:
        OFFSET = 1_0000
        # 以 n 结尾的最长长度
        length = [0 for i in range(2_0000 + 1)]
        ans = 0
        for n in arr:
            # 判断 n - diff 出现过没有
            if 0 <= n - difference + OFFSET < len(length):
                length[n + OFFSET] = 1 + length[n - difference + OFFSET]
            else:
                length[n + OFFSET] = 1
            ans = max(ans, length[n + OFFSET])
        return ans


# @lc code=end
if __name__ == '__main__':
    def test(arr, diff, expected):
        calc = Solution().longestSubsequence(arr, diff)
        if calc != expected:
            print(f'case failed: `{arr, diff}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1, 2, 3, 4], 1, 4)
    test([1, 3, 5, 7], 1, 1)
    test([1, 5, 7, 8, 5, 3, 4, 2, 1], -2, 4)

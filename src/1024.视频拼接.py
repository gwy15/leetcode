#
# @lc app=leetcode.cn id=1024 lang=python3
#
# [1024] 视频拼接
#
from typing import List
from utils import *
# @lc code=start

INF = 1 << 31


class Solution:
    def videoStitching(self, clips: List[List[int]], T: int) -> int:
        # sort by start
        clips.sort(key=lambda c: c[0])
        # the clips needed for [0, _]
        dp = [INF for i in range(T+1)]
        dp[0] = 0
        i = 0  # index of clips
        for e in range(0, T):
            if dp[e] == INF:  # unreachable
                return -1

            # from [, e] => [, clip_i.end]
            # for all clips that start at [e]
            while i < len(clips) and clips[i][0] == e:
                for reachable in range(e, clips[i][1] + 1):
                    if reachable <= T:
                        dp[reachable] = min(dp[reachable], dp[e] + 1)
                    else:
                        break
                i += 1

        if dp[T] == INF:
            return -1
        return dp[T]


# @lc code=end
if __name__ == '__main__':
    def test(input, T, expected):
        calc = Solution().videoStitching(input, T)
        if calc != expected:
            print(f'case failed: `{input}, {T}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([[0, 2], [4, 6], [8, 10], [1, 9], [1, 5], [5, 9]], 10, 3)
    test([[0, 1], [1, 2]], 5, -1)
    test([[0, 1], [6, 8], [0, 2], [5, 6], [0, 4], [0, 3], [6, 7], [1, 3], [
         4, 7], [1, 4], [2, 5], [2, 6], [3, 4], [4, 5], [5, 7], [6, 9]], 9, 3)
    test([[0, 5], [6, 8]], 7, -1)

#
# @lc app=leetcode.cn id=1025 lang=python3
#
# [1025] 除数博弈
#

# @lc code=start


class Solution:
    def divisorGame(self, N: int) -> bool:
        return N % 2 == 0
        # first_win = [None] * (N + 1)
        # first_win[1] = False
        # for n in range(2, N+1):
        #     #
        #     for p in range(1, n):
        #         if n % p != 0:
        #             continue
        #         if first_win[n - p] == False:
        #             first_win[n] = True
        #     if first_win[n] is None:
        #         first_win[n] = False

        # return first_win[N]


# @lc code=end
if __name__ == "__main__":
    f = Solution().divisorGame
    n = [
        None,  # 0
        # 1 输, 2 胜, ...
        False, True, False, True
    ]
    for i in range(1, len(n)):
        assert f(i) == n[i]
    assert f(1000) == True

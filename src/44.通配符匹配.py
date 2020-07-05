#
# @lc app=leetcode.cn id=44 lang=python3
#
# [44] 通配符匹配
#

import unittest
# @lc code=start


class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        m, n = len(s), len(p)

        # empty pattern
        last_dp = [False] * (m + 1)
        last_dp[0] = True
        # print(f'dp[0] = {last_dp}')

        for j in range(1, n+1):
            dp = [False] * (m + 1)
            # generate prefix from last_dp
            prefix = [False] * (m + 1)
            prefix[0] = last_dp[0]
            for i in range(1, m+1):
                prefix[i] = prefix[i-1] or last_dp[i]
            # print(f'  prefix = {prefix}')

            for i in range(m+1):
                char_p = p[j - 1]

                # regular match
                if ord('a') <= ord(char_p) <= ord('z'):
                    if i == 0:
                        dp[i] = False
                    elif char_p == s[i-1]:
                        dp[i] = last_dp[i-1]
                    else:
                        dp[i] = False
                # ? match
                elif char_p == '?':
                    dp[i] = last_dp[i-1] if i != 0 else False
                # * match
                else:
                    # dp[i] = max(last_dp[..k] for k in range(i))
                    dp[i] = prefix[i]
            last_dp = dp
            # print(f'dp[{j}] = {last_dp}')

        return last_dp[m]


# @lc code=end


class Test(unittest.TestCase):
    def test(self):
        def f(s, p, ans):
            print(f'testing {s} {p}')
            self.assertEqual(
                Solution().isMatch(s, p),
                ans
            )
        f('aa', 'a', False)
        f('aa', '*', True)
        f('cb', '?a', False)
        f('adceb', '*a*b', True)
        f('acdcb', 'a*c?b', False)
        f('', '', True)
        f('aaaaabaaaa', 'a*ab*a****', True)
        f('a', '**********', True)


if __name__ == "__main__":
    unittest.main()

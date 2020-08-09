#
# @lc app=leetcode.cn id=93 lang=python3
#
# [93] 复原IP地址
#
from typing import List
# @lc code=start


class Solution:
    def restoreIpAddresses(self, s: str) -> List[str]:
        n = len(s)
        # dp[i, j]: s[:i] 分成 j 个数字的组合
        dp = [[[] for _ in range(5)] for _ in range(n+1)]
        dp[0][0] = ['']
        for i in range(1, n+1):
            for j in range(1, 5):
                # 每个数字至少有一位，最多3位
                if not (j <= i <= 3*j):
                    continue
                results = []
                # 尝试每一位
                for length in (1, 2, 3):
                    # [i-length:i]
                    start = i - length
                    if start < 0:
                        break
                    num = s[start:i]
                    if len(num) > 1 and num.startswith('0'):
                        continue
                    if int(num) > 255:
                        break

                    last: str = dp[start][j-1]
                    for split in last:
                        if split:
                            results.append(split + '.' + num)
                        else:
                            results.append(num)

                # print(f'{s[:i]} ({i=}, {j=}) => {results}')
                dp[i][j] = results

        return dp[n][4]


# @lc code=end
if __name__ == "__main__":
    def f(s, ans):
        a = Solution().restoreIpAddresses(s)
        assert set(a) == set(ans)
    f('25525511135', ["255.255.11.135", "255.255.111.35"])
    f('255255255255', ['255.255.255.255'])
    f('0000', ['0.0.0.0'])
    f('12345', ['1.2.3.45', '1.2.34.5', '1.23.4.5', '12.3.4.5'])
    f('123', [])
    f('', [])
    f('1234', ['1.2.3.4'])
    f('010010', ['0.10.0.10', '0.100.1.0'])

#
# @lc app=leetcode.cn id=6 lang=python3
#
# [6] Z 字形变换
#

# @lc code=start


class Solution:
    def convert(self, s: str, numRows: int) -> str:
        LENGTH = len(s)
        N = numRows
        if N == 1:
            return s
        ans = []

        for n in range(N):
            if n == 0 or n == N-1:
                gap = 2 * N - 2
                ans.extend(s[n: LENGTH: gap])
            else:
                i = 0
                while True:
                    left = i * (2 * N - 2)
                    if left + n < LENGTH:
                        ans.append(s[left + n])
                    else:
                        break

                    right = (i + 1) * (2 * N - 2)
                    if right - n < LENGTH:
                        ans.append(s[right - n])
                    else:
                        break

                    i += 1

        return ''.join(ans)


# @lc code=end
if __name__ == "__main__":
    s = Solution()
    f = s.convert
    assert f('LEETCODEISHIRING', 3) == 'LCIRETOESIIGEDHN'
    assert f('LEETCODEISHIRING', 4) == 'LDREOEIIECIHNTSG'
    assert f('', 1) == ''
    assert f('test', 1) == 'test'

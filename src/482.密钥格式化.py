#
# @lc app=leetcode.cn id=482 lang=python3
#
# [482] 密钥格式化
#

# @lc code=start


class Solution:
    def licenseKeyFormatting(self, S: str, K: int) -> str:
        # import string
        # ok = string.ascii_letters + string.digits
        # letters = []
        # for s in S:
        #     if s in ok:
        #         letters.append(s.upper())
        letters = S.replace('-', '').upper()

        ans = []
        n = len(letters)
        i = n % K
        if i != 0:
            ans.append(''.join(letters[:n % K]))
        while i < n:
            ans.append(''.join(letters[i:i+K]))
            i += K
        return '-'.join(ans)


# @lc code=end
if __name__ == "__main__":
    s = Solution()
    f = s.licenseKeyFormatting
    assert f("5F3Z-2e-9-w", 4) == "5F3Z-2E9W"
    assert f("2-5g-3-J", 2) == "2-5G-3J"

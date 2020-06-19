#
# @lc app=leetcode.cn id=125 lang=python3
#
# [125] 验证回文串
#

# @lc code=start


class Solution:
    def isPalindrome(self, s: str) -> bool:

        n = len(s)
        left, right = 0, n-1
        while left < right:
            # get left
            while left < n and not s[left].isalnum():
                left += 1
            if left >= right:
                break
            left_char = s[left].lower()
            # get right
            while right >= 0 and not s[right].isalnum():
                right -= 1
            if left >= right:
                break
            right_char = s[right].lower()
            if left_char != right_char:
                return False
            else:
                left += 1
                right -= 1
        return True


# @lc code=end
if __name__ == "__main__":
    s = Solution()
    assert s.isPalindrome("A man, a plan, a canal: Panama")
    assert s.isPalindrome("race a car") is False

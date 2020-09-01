#
# @lc app=leetcode.cn id=367 lang=python3
#
# [367] 有效的完全平方数
#

# @lc code=start
class Solution:
    def isPerfectSquare(self, num: int) -> bool:
        # [left, right]
        left, right = 1, num
        while left < right:
            mid = (left + right) // 2
            n = mid * mid
            if n == num:
                return True
            elif n < num:
                left = mid + 1
            else:
                right = mid - 1
        return left * left == num


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().isPerfectSquare(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(1, True)
    test(16, True)
    test(14, False)

#
# @lc app=leetcode.cn id=1780 lang=python3
#
# [1780] 判断一个数字是否可以表示成三的幂的和
#

# @lc code=start
class Solution:
    def checkPowersOfThree(self, n: int) -> bool:
        # 判断数字是否在三进制下只有 1
        while n:
            k = n % 3
            if k == 2:
                return False
            n = n // 3
        return True


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().checkPowersOfThree(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(12, True)
    test(91, True)
    test(21, False)

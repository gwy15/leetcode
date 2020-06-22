#
# @lc app=leetcode.cn id=397 lang=python3
#
# [397] 整数替换
#

# @lc code=start


class SolutionDummy:
    def __init__(self):
        self.cache = None
        self.cache_size = None

    def integerReplacement(self, n: int) -> int:
        if self.cache is None:
            self.cache_size = min(3 * n, 100_000)
            self.cache = [None] * self.cache_size

        if n == 1:
            return 0

        if n < self.cache_size:
            # visiting (loop detected or cached before)
            if self.cache[n] is not None:
                return self.cache[n]

            # mark as visiting
            self.cache[n] = -1

        if n % 2 == 0:
            ans = 1 + self.integerReplacement(n // 2)
        else:
            steps1 = self.integerReplacement(n + 1)
            steps2 = self.integerReplacement(n - 1)
            if steps1 == -1:
                ans = 1 + steps2
            elif steps2 == -1:
                ans = 1 + steps1
            else:
                ans = 1 + min(steps1, steps2)
            # ans must not be -1 (at least 1 solution can be found)
        if n < self.cache_size:
            self.cache[n] = ans
        return ans


class Solution:
    def integerReplacement(self, n: int) -> int:
        ans = 0
        while n != 1:
            if n % 2 == 0:
                n //= 2
            elif n == 3:
                n -= 1
            elif (n + 1) % 4 == 0:
                n += 1
            else:
                n -= 1
            ans += 1

        return ans


# @lc code=end
if __name__ == "__main__":
    s = Solution()

    def f(a, b):
        c = s.integerReplacement(a)
        print(c)
        assert c == b
    f(8, 3)
    f(7, 4)
    f(100000000, 31)

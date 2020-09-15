

class Solution:
    def minimumOperations(self, leaves: str) -> int:
        inf = float('inf')
        dp = [inf, inf, inf]

        for i, ch in enumerate(leaves):
            this = 0 if ch == 'r' else 1

            if this == 0:
                dp = [dp[0], min(dp[0], dp[1]) + 1, min(dp[1], dp[2])]
                if i == 0:
                    dp[0] = 0
            else:
                dp = [dp[0] + 1, min(dp[0], dp[1]), min(dp[1], dp[2]) + 1]
                if i == 0:
                    dp[0] = 1

        return dp[2]


if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().minimumOperations(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test("rrryyyrryyyrr", 2)
    test("ryr", 0)
    test('rrrrrrrrrrrrrrr', 1)
    test('yyy', 2)
    test('yyyyyyyy', 2)
    test('ryyyy', 1)
    test('yyyyr', 1)
    test('rrr', 1)

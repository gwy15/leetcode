

class Solution:
    def minimumOperations(self, leaves: str) -> int:
        inf = float('inf')
        dp_0 = 0
        dp_1 = inf
        dp_2 = inf

        for i, ch in enumerate(leaves):
            this = 0 if ch == 'r' else 1

            if this == 0:
                _dp_0 = dp_0     # 0000 -> 0
                _dp_1 = min(
                    dp_0 + 1,
                    dp_1 + 1,
                )
                _dp_2 = min(dp_2, dp_1)
            else:
                _dp_0 = dp_0 + 1  # 翻转一次
                _dp_1 = min(dp_1, dp_0)
                if i == 0:
                    _dp_1 = inf
                _dp_2 = min(
                    dp_2 + 1,  # 翻转
                    dp_1 + 1  # 1 结尾翻转
                )
            # print(
            #     f'after string {leaves[:i]}: {dp_0, dp_1, dp_2} => {_dp_0, _dp_1, _dp_2}')
            dp_0, dp_1, dp_2 = _dp_0, _dp_1, _dp_2

        return dp_2


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

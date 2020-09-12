class Solution:
    def calculate(self, s: str) -> int:
        x, y = 1, 0
        for ch in s:
            if ch == 'A':
                x = 2 * x + y
            elif ch == 'B':
                y = 2 * y + x
        return x + y


if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().calculate(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('AB', 4)

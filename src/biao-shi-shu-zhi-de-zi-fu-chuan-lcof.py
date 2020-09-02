import re


class Solution:
    def isNumber(self, s: str) -> bool:
        regex = r'^\s*[+-]?((\d+(\.\d*)?)|(\d*\.\d+))([eE][+-]?\d+)?\s*$'
        return re.match(regex, s) != None


if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().isNumber(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    trues = [
        '0', '+10', '-10', '-0',
        '0.1', '.1', '10.', '10.1',
        '+.1', '-1.', '+1.5',
        '5e2', '-1E-16', '0123',
        '1 ', ' 1'
    ]
    falses = [
        '', '12e', '1a2.3', '1.2.3', '+-5', '12e+5.4',
        '.e3', '.1.2'
    ]
    for s in trues:
        test(s, True)
    for s in falses:
        test(s, False)

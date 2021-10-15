#
# @lc app=leetcode.cn id=38 lang=python3
#
# [38] 外观数列
#

# @lc code=start
from typing import Generator, Tuple


def groups(s: str) -> Generator[Tuple[str, int], None, None]:
    last_char = None
    count = 0
    for ch in s:
        if ch == last_char:
            count += 1
        else:
            if count > 0:
                yield (last_char, count)
            last_char = ch
            count = 1
    if count > 0:
        yield (last_char, count)


def f(i: str) -> str:
    s = []
    for ch, times in groups(i):
        s.append(str(times))
        s.append(ch)
    return ''.join(s)


class Solution:
    def countAndSay(self, n: int) -> str:
        s = '1'
        for _ in range(n-1):
            s = f(s)
        return s


# @lc code=end
if __name__ == '__main__':

    def test(input, expected):
        calc = Solution().countAndSay(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(1, '1')
    test(2, '11')
    test(3, '21')
    test(4, '1211')
    test(5, '111221')

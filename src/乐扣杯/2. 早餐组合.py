from typing import List
from utils import *


#
import bisect


class Solution:
    def breakfastNumber(self, staple: List[int], drinks: List[int], x: int) -> int:
        staple.sort()
        drinks.sort()
        #
        count = 0
        for i, money in enumerate(staple):
            if money >= x:
                break
            # find out how much drinks can he has
            available = bisect.bisect_right(drinks, x - money)
            count = (count + available) % 1000000007

        return count


# test
if __name__ == '__main__':
    def test(staple, drinks, x, expected):
        calc = Solution().breakfastNumber(staple, drinks, x)
        if calc != expected:
            print(f'case failed: `{staple, drinks, x}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(staple=[10, 20, 5], drinks=[5, 5, 2], x=15, expected=6)
    test(staple=[2, 1, 1], drinks=[8, 9, 5, 1], x=9, expected=8)

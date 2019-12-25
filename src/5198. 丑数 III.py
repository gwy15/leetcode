from math import gcd
import functools


def lcm(a, b):
    return (a // gcd(a, b)) * b


def uglyN(n, a, b, c, lcm_ab, lcm_bc, lcm_ac, lcm_abc):
    return n // a + n // b + n // c - (
        n // lcm_ab + n // lcm_bc + n // lcm_ac
    ) + n // lcm_abc


class Solution:
    def nthUglyNumber(self, n: int, a: int, b: int, c: int) -> int:
        lcm_ab = lcm(a, b)
        lcm_bc = lcm(b, c)
        lcm_ac = lcm(a, c)
        lcm_abc = lcm(lcm_ab, c)

        N = functools.partial(uglyN, a=a, b=b, c=c, lcm_ab=lcm_ab,
                              lcm_bc=lcm_bc, lcm_ac=lcm_ac, lcm_abc=lcm_abc)

        start, end = 0, 2*(10**9)  # (start, end]
        # search until ... start(n-1 th), end=start+1 (n th), ...
        # answer will be end
        while start + 1 < end:  # binary search
            # promise end - start >= 2
            mid = start + (end - start) // 2  # (start, mid] (mid, end]
            # find nth for mid
            midN = N(mid)
            if midN < n:  # right
                start = mid  # (mid, end]
            else:  # midN >= n:
                end = mid  # (start, mid]
        return end


print(Solution().nthUglyNumber(3, 2, 3, 5))
print(Solution().nthUglyNumber(4, 2, 3, 4))
print(Solution().nthUglyNumber(5, 2, 11, 13))
print(Solution().nthUglyNumber(1000000000, 2, 217983653, 336916467))

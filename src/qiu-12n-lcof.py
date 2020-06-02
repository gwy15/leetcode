def _if(cond, a, b):
    return (cond and a) or b


def multiply(a, b):
    ans = 1

    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1
    ans += _if(b & 1, a, 0)
    a <<= 1
    b >>= 1

    return ans


class Solution:
    def sumNums(self, n: int) -> int:
        return multiply(n, n+1) >> 1


class Test:
    def test(self):
        for n in range(1000):
            assert Solution().sumNums(n) == n * (n+1)/2

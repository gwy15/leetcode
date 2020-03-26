#
# @lc app=leetcode.cn id=60 lang=python3
#
# [60] 第k个排列
#

from typing import List

# @lc code=start


def cantor_expansion(k: List[int]) -> int:
    result: int = 0
    factorial: int = 1
    N: int = len(k)
    for i in range(N-1, -1, -1):  # from end to begin
        a_i = sum(k[i] > k[j] for j in range(i+1, N))
        result += a_i * factorial
        factorial *= (N - i)
    return result


def cantor_expansion_reverse(index: int, N: int) -> List[int]:
    permutation = [None] * N
    for i in range(1, N+1):
        permutation[N - i] = index % i  # a_i
        index //= i
    # convert a_i to k_i
    options = list(range(N))
    for i in range(N):
        a_i = permutation[i]
        permutation[i] = options.pop(a_i)
    return permutation


class Solution:
    def getPermutation(self, n: int, k: int) -> str:
        return ''.join(
            str(i+1) for i in
            cantor_expansion_reverse(k-1, n))


# @lc code=end

def test():
    from itertools import permutations
    N = 5
    for index, perm in enumerate(permutations(range(N))):
        assert cantor_expansion(perm) == index
        assert list(perm) == cantor_expansion_reverse(index, N)


if __name__ == "__main__":
    # test()
    print(Solution().getPermutation(3, 3))
    print(Solution().getPermutation(4, 9))

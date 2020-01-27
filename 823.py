from typing import List

from math import sqrt
MOD = 1_000_000_007


class Solution:
    def numFactoredBinaryTrees(self, A: List[int]) -> int:
        A.sort()
        num_to_index = {
            n: i
            for i, n in enumerate(A)
        }

        f = [0] * len(A)
        for i, n in enumerate(A):
            cnt = 1  # at least 1
            used = set()
            for index_a in range(i):  # sub nums
                a = A[index_a]
                if n % a == 0 and a not in used:
                    used.add(a)
                    b = n // a
                    if b == a:  # n, a, a
                        cnt += (f[index_a] * f[index_a]) % MOD
                    elif b in num_to_index:
                        index_b = num_to_index[b]
                        cnt += (2 * f[index_a] * f[index_b]) % MOD
                        used.add(b)
                    else:  # not in nums
                        pass

                if a >= sqrt(n):
                    break
            f[i] = cnt
        # print(A)
        # print(f)
        return sum(f) % MOD


if __name__ == "__main__":
    assert(
        Solution().numFactoredBinaryTrees(
            [18, 3, 6, 2]
        ) == 12)

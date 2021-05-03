import functools


class Solution:
    def minNumber(self, nums: List[int]) -> str:
        def cmp(s1, s2):
            if s1 + s2 > s2 + s1:
                return 1
            else:
                return -1
        return ''.join(
            sorted(
                map(str, nums),
                key=functools.cmp_to_key(cmp)
            )
        )

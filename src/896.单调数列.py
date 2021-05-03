#
# @lc app=leetcode.cn id=896 lang=python3
#
# [896] 单调数列
#
from prelude import *
# @lc code=start
class Solution:
    def isMonotonic(self, A: List[int]) -> bool:
        # 0 for unknown, 1 for asc, -1 for desc
        status = 0
        for i in range(len(A) - 1):
            if status == 0:
                if A[i] == A[i+1]:
                    status = 0
                elif A[i] < A[i+1]:
                    status = 1
                else:
                    status = -1
            elif status == 1:
                if A[i] <= A[i+1]:
                    continue
                else:
                    return False
            else:
                if A[i] >= A[i+1]:
                    continue
                else:
                    return False
        return True

# @lc code=end

if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().isMonotonic(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([1,2,2,3], True)
    test([6,5,4,4], True)
    test([1,1,1], True)
    test([1,1,1,2,1], False)

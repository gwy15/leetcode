#
# @lc app=leetcode.cn id=39 lang=python3
#
# [39] 组合总和
#
from typing import List
# @lc code=start


class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        answers = [[] for _ in range(target + 1)]

        for x in candidates:
            if x > target:
                continue
            # 0 -> [x]
            answers[x].append([x])
            # other
            for start in range(1, target - x + 1):
                for original_solution in answers[start]:
                    solution = original_solution + [x]
                    answers[start + x].append(solution)

        return answers[target]


# @lc code=end
if __name__ == "__main__":
    def test(can, tar, ans):
        o = Solution().combinationSum(can, tar)
        print(o)
        def f(items): return set(','.join(map(str, _)) for _ in items)
        assert f(o) == f(ans)

    test([2, 3, 6, 7], 7, [[7], [2, 2, 3]])
    test([2, 3, 5], 8, [
        [2, 2, 2, 2],
        [2, 3, 3],
        [3, 5]
    ])
    test([2], 1, [])

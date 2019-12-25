#
# @lc app=leetcode id=851 lang=python3
#
# [851] Loud and Rich
#
from typing import List


class Solution:
    def loudAndRich(self, richer: List[List[int]], quiet: List[int]) -> List[int]:
        N = len(quiet)
        answer = [None] * N
        # build the tree
        edges = {}  # points to those richer
        for child, root in richer:
            if root in edges:
                edges[root].add(child)
            else:
                edges[root] = {child}

        def key(i): return quiet[i]

        def _getAnswerFor(i):
            if answer[i] is None:  # not calculated
                answer[i] = i  # default as self
                if i in edges:
                    answer[i] = min(i,
                                    min((_getAnswerFor(child)
                                         for child in edges[i]), key=key),
                                    key=key)
            return answer[i]

        # travel
        for i in range(N):
            _getAnswerFor(i)

        return answer


# print(Solution().loudAndRich(
#     [],
#     [0]
# ))

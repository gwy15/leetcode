#
# @lc app=leetcode.cn id=332 lang=python3
#
# [332] 重新安排行程
#
from typing import List
from utils import *
# @lc code=start
from sortedcontainers import SortedList
from collections import defaultdict


class Solution:
    def findItinerary(self, tickets: List[List[str]]) -> List[str]:
        edges = defaultdict(SortedList)
        for start, end in tickets:
            edges[start].add(end)

        ans = []

        def dfs(node):
            while edges[node]:
                _next = edges[node].pop(0)
                dfs(_next)
            ans.append(node)

        dfs('JFK')

        return ans[::-1]


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().findItinerary(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        [["MUC", "LHR"], ["JFK", "MUC"], ["SFO", "SJC"], ["LHR", "SFO"]],
        ["JFK", "MUC", "LHR", "SFO", "SJC"]
    )
    test(
        [["JFK", "SFO"], ["JFK", "ATL"], ["SFO", "ATL"],
            ["ATL", "JFK"], ["ATL", "SFO"]],
        ["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
    )
    test(
        [["JFK", "KUL"], ["JFK", "NRT"], ["NRT", "JFK"]],
        ["JFK", "NRT", "JFK", "KUL"]
    )

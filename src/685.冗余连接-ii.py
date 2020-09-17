#
# @lc app=leetcode.cn id=685 lang=python3
#
# [685] 冗余连接 II
#
from typing import List
from utils import *
# @lc code=start


class UnionFind:
    def __init__(self, n):
        self.parent = [i for i in range(n+1)]

    def find(self, i):
        if self.parent[i] != i:
            self.parent[i] = self.find(self.parent[i])
        return self.parent[i]

    def union(self, i, j):
        p1 = self.find(i)
        p2 = self.find(j)
        if p1 == p2:
            return True
        else:
            self.parent[p1] = self.parent[p2]
            return False


class Solution:
    def findRedundantDirectedConnection(self, edges: List[List[int]]) -> List[int]:
        n = len(edges)

        # 两种情况：
        # A. 存在一个点入度为 0 ，一个点入度为 2
        # B. 入度都是 1，则构成环

        uf = UnionFind(n)
        parent = list(range(n+1))
        multi_in_edges = []
        last_circle_edge = []

        for i, (a, b) in enumerate(edges):
            if parent[b] != b:
                # b 已经有父节点了，则出现一个入度为 2 的点。这个分支只会出现一次
                # 如果先检测到回路，则返回第一条边
                multi_in_edges.append([parent[b], b])
                # 否则，返回第二条边（最后出现的边）
                multi_in_edges.append([a, b])
                # 不把 a -> b 加入并查集。如果没有检测到环，那删除 a -> b 即可；
                # 如果依然检测到环，那返回第一条边
            else:
                # 没有父节点，可以连接
                parent[b] = a
                # 是否是同一个根？若是，则构成回路（b 的根就是自己）
                same_root = uf.union(a, b)
                if same_root:
                    last_circle_edge = [a, b]

        # 没有出现入度为 2 的情况，返回检测到环的边
        if multi_in_edges == []:
            return last_circle_edge

        # 检测到环了吗？
        if last_circle_edge:
            return multi_in_edges[0]
        else:
            return multi_in_edges[1]


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().findRedundantDirectedConnection(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test([[1, 2], [1, 3], [2, 3]], [2, 3])
    test([[1, 2], [2, 3], [3, 4], [4, 1], [1, 5]], [4, 1])

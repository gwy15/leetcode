from typing import List


class Solution:
    def minReorder(self, n: int, connections: List[List[int]]) -> int:
        from_to = [[] for _ in range(n)]
        to_from = [[] for _ in range(n)]
        for a, b in connections:
            from_to[a].append(b)
            to_from[b].append(a)

        seen = [False] * n
        # begin dfs
        counter = 0
        stack = [0]
        seen[0] = True
        while len(stack):
            begin = stack.pop()
            for next in from_to[begin]:
                if not seen[next]:
                    seen[next] = True
                    stack.append(next)
                    counter += 1
            for prev in to_from[begin]:
                if not seen[prev]:
                    seen[prev] = True
                    stack.append(prev)

        return counter


if __name__ == "__main__":
    f = Solution().minReorder
    assert f(6, [[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]]) == 3
    assert f(5, [[1, 0], [1, 2], [3, 2], [3, 4]]) == 2
    assert f(3,  [[1, 0], [2, 0]]) == 0

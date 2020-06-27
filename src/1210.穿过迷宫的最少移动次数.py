#
# @lc app=leetcode.cn id=1210 lang=python3
#
# [1210] 穿过迷宫的最少移动次数
#
from unittest import TestCase, main
from typing import List
# @lc code=start
import queue
from dataclasses import dataclass


@dataclass(eq=True, unsafe_hash=False)
class State:
    x: int
    y: int
    direction: int

    def __hash__(self) -> int:
        # n ~ 100, 7 bits is enough
        # direction can take 1 bit
        return (self.x << 8) + (self.y << 1) + self.direction
        # return (self.x << 8) + (self.y << 1) + (
        #     1 if self.direction == 'X' else 0
        # )


class Solution:
    def minimumMoves(self, grid: List[List[int]]) -> int:
        X_Dir = 1
        Y_Dir = 0
        n = len(grid)

        q = queue.Queue()
        seen = set()

        def check(s):
            if s not in seen:
                seen.add(s)
                q.put(s)
        # BFS
        check(State(0, 0, Y_Dir))
        steps = 0

        while q.qsize():
            for i in range(q.qsize()):
                s: State = q.get()
                x, y, direction = s.x, s.y, s.direction
                if x == n-1 and y == n-2 and direction == Y_Dir:
                    return steps

                if direction == X_Dir:
                    # go x
                    if x + 2 < n and grid[x+2][y] == 0:  # x + 2 is the next head
                        check(State(x+1, y, X_Dir))
                    # can go y or rotate
                    if y + 1 < n and grid[x][y+1] == 0 and grid[x+1][y+1] == 0:
                        check(State(x, y, Y_Dir))
                        check(State(x, y+1, X_Dir))
                else:  # vertical
                    # go y
                    if y + 2 < n and grid[x][y+2] == 0:
                        check(State(x, y+1, Y_Dir))
                    # rotate or go x
                    if x + 1 < n and grid[x+1][y] == 0 and grid[x+1][y+1] == 0:
                        check(State(x, y, X_Dir))
                        check(State(x+1, y, Y_Dir))
            steps += 1

        return -1


# @lc code=end


class Test(TestCase):
    def test(self):
        f = Solution().minimumMoves
        self.assertEqual(f(
            [[0, 0, 0, 0, 0, 1],
             [1, 1, 0, 0, 1, 0],
             [0, 0, 0, 0, 1, 1],
             [0, 0, 1, 0, 1, 0],
             [0, 1, 1, 0, 0, 0],
             [0, 1, 1, 0, 0, 0]]
        ), 11)
        self.assertEqual(f(
            [[0, 0, 1, 1, 1, 1],
             [0, 0, 0, 0, 1, 1],
             [1, 1, 0, 0, 0, 1],
             [1, 1, 1, 0, 0, 1],
             [1, 1, 1, 0, 0, 1],
             [1, 1, 1, 0, 0, 0]]
        ), 9)


if __name__ == "__main__":
    main()

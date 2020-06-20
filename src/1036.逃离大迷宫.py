#
# @lc app=leetcode.cn id=1036 lang=python3
#
# [1036] 逃离大迷宫
#
from typing import List
# @lc code=start
from queue import Queue


class Found(Exception):
    pass


def catch_found(f):
    def new(*args, **kws):
        try:
            return f(*args, **kws)
        except Found:
            return True
    return new


def neighbor(pt):
    x, y = pt
    return (
        (x+1, y), (x, y+1), (x-1, y), (x, y-1)
    )


class Solution:
    @catch_found
    def isEscapePossible(self, blocked: List[List[int]], source: List[int], target: List[int]) -> bool:
        N = 10 ** 6
        nb = len(blocked)
        area = nb * nb // 2
        blocked = set(map(tuple, blocked))
        source = tuple(source)
        target = tuple(target)

        def escapable(p1, p2):
            seen = set()
            queue = Queue()
            queue.put(p1)
            seen.add(p1)
            while queue.qsize():
                pt = queue.get()
                if pt == p2:
                    raise Found
                # neighbors
                for next_pt in neighbor(pt):
                    x, y = next_pt
                    in_grid = 0 <= x <= N and 0 <= y <= N
                    if not in_grid:
                        continue
                    if (next_pt not in seen) and (next_pt not in blocked):
                        queue.put(next_pt)
                        seen.add(next_pt)
                        if len(seen) > area:
                            return True
            return False

        return escapable(source, target) and escapable(target, source)


# @lc code=end

if __name__ == "__main__":
    s = Solution()
    assert s.isEscapePossible(
        blocked=[[0, 1], [1, 0]], source=[0, 0], target=[0, 2]
    ) is False

    assert s.isEscapePossible(
        blocked=[], source=[0, 0], target=[999999, 999999]
    ) is True

    assert s.isEscapePossible(
        [[629173, 232687], [695376, 426436], [164553, 460497], [956954, 310238], [322182, 515846], [847558, 240198], [792551, 770487], [4146, 573081], [
            397773, 751953], [899296, 153588], [731578, 860293], [616566, 678204], [731801, 813001], [68606, 163479], [839668, 109780], [57126, 967537]],
        [603201, 575992],
        [765501, 302566],
    ) is True

    assert s.isEscapePossible(
        [[0, 3], [1, 0], [1, 1], [1, 2], [1, 3]],
        [0, 0],
        [0, 2],
    ) is True

    import cProfile
    import random
    N = 1_000_000
    def p(): return [random.randrange(N), random.randrange(N)]
    blocked = [
        p()
        for _ in range(200)
    ]
    source = p()
    target = p()
    cProfile.run('s.isEscapePossible(blocked, source, target)', sort='tottime')

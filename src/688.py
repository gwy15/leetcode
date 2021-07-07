
DELTAS = [
    (2, 1), (2, -1),
    (-2, 1), (-2, -1),
    (1, 2), (1, -2),
    (-1, 2), (-1, -2)
]

class Solution:
    def new_map(self, n):
        return [
            [0.0 for __ in range(n)]
            for _ in range(n)
        ]
    def knightProbability(self, n: int, k: int, row: int, column: int) -> float:
        map = self.new_map(n)
        map[row][column] = 1.0
        for _  in range(k):
            new_map = self.new_map(n)
            for x in range(n):
                for y in range(n):
                    for dx, dy in DELTAS:
                        new_x, new_y = x + dx, y + dy
                        if 0 <= new_x < n and 0 <= new_y < n:
                            new_map[new_x][new_y] += map[x][y] / 8
            map = new_map
        return sum(sum(r) for r in map)

if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().knightProbability(*input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(
        (3,2,0,0),
        0.0625
    )


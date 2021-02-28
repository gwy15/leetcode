#
# @lc app=leetcode.cn id=1079 lang=python3
#
# [1079] 活字印刷
#

# @lc code=start
class Solution:
    def numTilePossibilities(self, tiles: str) -> int:
        n, tiles = len(tiles), sorted(tiles)

        def backtrace(used):
            ans = 0
            for i in range(n):
                if used[i]:
                    continue
                # use this one
                if i > 0 and tiles[i-1] == tiles[i] and not used[i-1]:
                    continue
                used[i] = True
                ans += 1
                ans += backtrace(used)
                used[i] = False
            return ans
        return backtrace([False] * n)


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().numTilePossibilities(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test('AAB', 8)
    test('AAABCB', 188)

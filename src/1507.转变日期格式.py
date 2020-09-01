#
# @lc app=leetcode.cn id=1507 lang=python3
#
# [1507] 转变日期格式
#

# @lc code=start
class Solution:
    def reformatDate(self, date: str) -> str:
        day, mon, year = date.split()
        if day[1].isdigit():
            day = int(day[:2])
        else:
            day = int(day[:1])
        mon = {
            'Jan': 1, "Feb": 2, "Mar": 3, "Apr": 4, "May": 5, "Jun": 6, "Jul": 7, "Aug": 8, "Sep": 9, "Oct": 10, "Nov": 11, "Dec": 12
        }[mon]
        return f'{year}-{mon:02}-{day:02}'


# @lc code=end
if __name__ == '__main__':
    def test(input, expected):
        calc = Solution().reformatDate(input)
        if calc != expected:
            print(f'case failed: `{input}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test("6th Jun 1933", "1933-06-06")

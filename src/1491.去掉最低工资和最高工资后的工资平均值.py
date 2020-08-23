#
# @lc app=leetcode.cn id=1491 lang=python3
#
# [1491] 去掉最低工资和最高工资后的工资平均值
#

# @lc code=start
class Solution:
    def average(self, salary: List[int]) -> float:
        n = len(salary)
        total = sum(salary)
        highest = max(salary)
        lowest = min(salary)
        return (total - highest - lowest) / (n - 2)

# @lc code=end

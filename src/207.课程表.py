#
# @lc app=leetcode.cn id=207 lang=python3
#
# [207] 课程表
#
from typing import List
# @lc code=start
from queue import Queue


class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        edges = [[] for _ in range(numCourses)]
        in_degree = [0 for _ in range(numCourses)]
        for b, a in prerequisites:
            edges[a].append(b)
            in_degree[b] += 1
        #
        zero_in_degree = Queue()
        for i, degree in enumerate(in_degree):
            if degree == 0:
                zero_in_degree.put(i)
        #
        finished_courses = 0
        while zero_in_degree.qsize() > 0:
            finish = zero_in_degree.get()
            # finish course
            finished_courses += 1
            for next_course in edges[finish]:
                in_degree[next_course] -= 1
                if in_degree[next_course] == 0:
                    zero_in_degree.put(next_course)
        return finished_courses == numCourses


# @lc code=end
if __name__ == "__main__":
    f = Solution().canFinish
    assert f(2, [[1, 0]])
    assert not f(2, [[1, 0], [0, 1]])

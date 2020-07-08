from typing import List


class Solution:
    def divingBoard(self, shorter: int, longer: int, k: int) -> List[int]:
        ans = []
        if k == 0:
            return ans
        a = k * shorter
        ans.append(a)
        if shorter != longer:
            for _ in range(k):
                a += longer - shorter
                ans.append(a)
        return ans


if __name__ == "__main__":
    f = Solution().divingBoard
    assert f(1, 1, 2) == [2]
    assert f(1, 2, 2) == [2, 3, 4]
    assert f(1, 2, 3) == [3, 4, 5, 6]
    assert f(1, 1, 0) == []

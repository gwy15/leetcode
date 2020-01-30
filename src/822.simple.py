class Solution:
    def flipgame(self, fronts: List[int], backs: List[int]) -> int:
        options = sorted(set(fronts) | set(backs))
        for num in options:
            for index in range(len(fronts)):
                if fronts[index] == num and backs[index] == num:
                    break
            else:
                return num
        return 0

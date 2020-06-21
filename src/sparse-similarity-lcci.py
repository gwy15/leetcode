from typing import List


class Solution:
    def computeSimilarities(self, docs: List[List[int]]) -> List[str]:
        docs = list(map(set, docs))
        ans = []
        for i in range(len(docs)):
            if len(docs[i]) == 0:
                continue
            for j in range(i + 1, len(docs)):
                if len(docs[j]) == 0:
                    continue
                intersec = len(docs[i] & docs[j])
                if intersec == 0:
                    continue

                union = len(docs[i]) + len(docs[j]) - intersec
                ans.append(f'{i},{j}: {intersec / union + 1e-9:.4f}')
        return ans


if __name__ == "__main__":
    s = Solution()
    assert s.computeSimilarities(
        [
            [14, 15, 100, 9, 3],
            [32, 1, 9, 3, 5],
            [15, 29, 2, 6, 8, 7],
            [7, 10]
        ]
    ) == [
        "0,1: 0.2500",
        "0,2: 0.1000",
        "2,3: 0.1429"
    ]

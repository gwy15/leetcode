class Solution:
    def compressString(self, S: str) -> str:
        c, count = '', 0
        compressed = []
        for ch in S + '_':
            if ch != c:
                if count:
                    compressed.append((c, count))
                c, count = ch, 1
            else:
                count += 1
        if sum(1 + len(str(i[1])) for i in compressed) >= len(S):
            return S
        else:
            return ''.join(f'{i[0]}{i[1]}' for i in compressed)


s = Solution().compressString('aabccccaaa')
print(s)

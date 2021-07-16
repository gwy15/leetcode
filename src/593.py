from prelude import *

class Solution:
    def validSquare(self, p1: List[int], p2: List[int], p3: List[int], p4: List[int]) -> bool:
        # 
        ps = [p1, p2, p3, p4]
        ps = sorted(ps, key=lambda p: p[0] * 100000 + p[1])
        return self.check(ps[0], ps[1], ps[2], ps[3])
    
    def check(self, p1, p2, p3, p4):
        def length(p1, p2):
            return (p1[0] - p2[0]) ** 2 + (p1[1] - p2[1]) ** 2
        def eq(p1, p2, p3, p4):
            return length(p1, p2) == length(p3, p4)
        # print(p1, p2, p3, p4)
        return length(p1, p2) > 0 and \
            eq(p1, p2, p2, p4) and eq(p2, p4, p1, p3) \
            and eq(p1, p3, p3, p4) \
            and eq(p1, p4, p2, p3)


if __name__ == '__main__':
    def test(p1, p2, p3,p4, expected):
        calc = Solution().validSquare(p1, p2, p3,p4)
        if calc != expected:
            print(f'case failed: `{p1, p2, p3,p4}`')
            print(f'  calc     = `{calc}`')
            print(f'  expected = `{expected}`')
            exit(1)
    test(p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,1], expected=True)
    test(p1=[0,0], p2=[1,1], p3=[1,0],p4=[0,12], expected=False)
class Solution:
    def findWords(self, words: List[str]) -> List[str]:
        rows = [
            set("qwertyuiop"),
            set("asdfghjkl"),
            set("zxcvbnm")
        ]
        mapper = {}
        for set_ in rows:
            for k in set_:
                mapper[k] = set_
                
        def ok(w):
            set_ = mapper[w[0].lower()]
            return all(k in set_ for k in w.lower())

        return list(filter(ok, words))

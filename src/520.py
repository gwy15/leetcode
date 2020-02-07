class Solution:
    def detectCapitalUse(self, word: str) -> bool:
        def ok(word):
            return word == word.upper() or word == word.lower() or word == word.capitalize()
        return all(ok(w) for w in word.split())

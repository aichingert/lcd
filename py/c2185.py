class Solution:
    def prefixCount(self, words: List[str], pref: str) -> int:
        return sum([1 if word.startswith(pref) else 0 for word in words])

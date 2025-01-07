class Solution:
    def stringMatching(self, words: List[str]) -> List[str]:
        ans = set()

        for word in words:
            for sec in words:
                if word != sec and sec in word:
                    ans.add(sec)
        return list(ans)

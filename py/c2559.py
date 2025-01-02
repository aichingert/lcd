class Solution:
    def vowelStrings(self, words: List[str], queries: List[List[int]]) -> List[int]:
        ans = []
        valid = []
        vowels = ["a", "e", "i", "o", "u"]
        cnt = 0
        for word in words:
            if word[0] in vowels and word[-1] in vowels:
                cnt += 1
            valid.append(cnt)
        
        for query in queries:
            cur = valid[query[1]]
            if query[0] > 0:
                cur -= valid[query[0] - 1]
            ans.append(cur)

        return ans

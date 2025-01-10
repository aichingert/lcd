class Solution:
    def wordSubsets(self, words1: List[str], words2: List[str]) -> List[str]:
        l = [0] * 26
        ans = []

        for word in words2:
            cur = [0] * 26

            for c in word:
                i = ord(c) - ord("a")
                cur[i] += 1
                l[i] = max(l[i], cur[i])

        for word in words1:
            val = True
            cur = [0] * 26

            for c in word:
                cur[ord(c) - ord("a")] += 1

            for i in range(len(l)):
                if l[i] > 0 and cur[i] < l[i]:
                    val = False
                    break

            if val:
                ans.append(word)

        return ans

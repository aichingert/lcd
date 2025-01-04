class Solution:
    def countPalindromicSubsequence(self, s: str) -> int:
        ans = 0
        fst = {}
        lst = {}

        for i in range(len(s)):
            if s[i] not in fst:
                fst[s[i]] = i
            lst[s[i]] = i

        for k in fst:
            u = 0
            for i in range(fst[k] + 1, lst[k]):
                if u & (1 << (ord(s[i]) - 97)) == 0:
                    ans += 1
                u |= 1 << (ord(s[i]) - 97)

        return ans

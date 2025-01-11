class Solution:
    def canConstruct(self, s: str, k: int) -> bool:
        h = {}
        o = 0
        e = 0

        for c in s:
            if c in h:
                h[c] += 1

                if h[c] % 2 == 0:
                    e += 1
                    o -= 1
                else:
                    o += 1
            else:
                h[c] = 1
                o += 1

        if o > k or o + e * 2 < k:
            return False
        return True

        

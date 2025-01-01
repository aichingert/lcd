class Solution:
    def maxScore(self, s: str) -> int:
        vals = []
        cur = [0, 0]

        for i in range(len(s) - 1):
            cur[int(s[i])] += 1
            vals.append(cur.copy())
        if s[-1] == "1":
            cur[1] += 1
        vals.append(cur.copy())

        ans = 0

        for val in vals:
            ans = max(ans, val[0] + (cur[1] - val[1]))

        return ans

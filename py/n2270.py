class Solution:
    def waysToSplitArray(self, nums: List[int]) -> int:
        ans = 0
        rhs = 0
        lhs = 0

        for n in nums:
            rhs += n
        
        for i in range(len(nums) - 1):
            lhs += nums[i]
            rhs -= nums[i]

            if lhs >= rhs:
                ans += 1

        return ans

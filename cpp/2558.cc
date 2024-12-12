// TODO: bad solution;

class Solution {
public:
    long long pickGifts(vector<int>& gifts, int k) {
        long long ans = 0;

        for (int i = 0; i < k; i++) {
            int max = 0;

            for (int j = 0; j < gifts.size(); j++) {
                if (gifts[j] > gifts[max]) {
                    max = j;
                }
            }

            gifts[max] = sqrt(gifts[max]);
        }

        for (auto& gift : gifts) {
            ans += gift;
        }

        return ans;
    }
};

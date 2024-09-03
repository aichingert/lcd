class Solution {
public:
    int getLucky(string s, int k) {
        int a = 0;

        for (char c : s) {
            int n = (c - 97 + 1);

            while (n > 0) {
                a += n % 10;
                n /= 10;
            }
        }

        for (int i = 1; i < k; i++) {
            int n = 0;

            while (a > 0) {
                n += a % 10;
                a /= 10;
            }

            a = n;
        }

        return a;
    }
};

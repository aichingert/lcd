class Solution {
public:
    bool lemonadeChange(vector<int>& bills) {
        int m[2] = {0,0};

        for (auto bill : bills){
            switch (bill) {
                case 5:
                    m[0]++;
                    break;
                case 10:
                    if (m[0] == 0) return false;
                    m[0]--;
                    m[1]++;
                    break;
                case 20:
                    if (m[0] > 0 && m[1] > 0) {
                        m[0]--;
                        m[1]--;
                    } else if (m[0] > 2) {
                        m[0] -= 3;
                    } else return false;
            }
        }

        return true;
    }
};

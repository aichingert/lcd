/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    vector<int> nodesBetweenCriticalPoints(ListNode* head) {
        vector<int> ans = {-1,-1};
        int fst = -1;
        int lst = -1;
        int i = 0;
        ListNode* prev = NULL;

        while (head != NULL) {
            if (prev != NULL && head->next != NULL) {
                if (prev->val > head->val && head->val < head->next->val
                || prev->val < head->val && head->val > head->next->val) {
                    if (lst == -1) {
                        lst = i;
                        fst = i;
                    } else {
                        ans[0] = ans[0] == -1 ? i - lst : std::min(ans[0], i - lst);
                        ans[1] = i - fst;
                        lst = i;
                    }
                }
            }

            i++;
            prev = head;
            head = head->next;
        }

        return ans;
    }
};
